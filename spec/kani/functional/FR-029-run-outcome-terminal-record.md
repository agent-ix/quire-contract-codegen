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
- CG defect: exactly the errors FR-029-AC-11 lists, which this repository raised and which carry no
  QSL catalog code, including `ReplayPackageError::InvalidFunction` and
  `FrameReplayError::Name`;
- setup refusal on data: a refusal of the replay setup that this repository reaches after the run
  was falsified, that is not a `ReplayRefusal` returned by `qsl_replay::replay`, and that carries
  a QSL catalog code: a `CallSiteRefusal` other than `Fault` (code from `CallSiteRefusal::code()`)
  and `DependencyLockError::Input` (code from `DependencyInputRefusal::code()`), each bare or
  wrapped in `ReplayPackageError` or `FrameReplayError`. A lock that repeats a dependency identity
  is one of these: QSL's `DependencyInput::new` refuses it as `DuplicateIdentity`, code
  `invalid_package` with cause `conflicting-definition`, and it arrives as
  `DependencyLockError::Input`.

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
  | `falsified` | setup refusal on data | `Inconclusive(InconclusiveCause::ReplayRefused)`, carrying the refusal's QSL catalog code |
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
- The Kani adapter shall put only a code from QSL's closed catalog inside
  `Inconclusive(InconclusiveCause::ReplayRefused)`, and shall define no code of its own for it.
- The Kani adapter shall map a failed replay setup to `Inconclusive(ReplayRefused)` carrying the
  QSL catalog code only when a QSL refusal value supplies that code: a `CallSiteRefusal` other than
  `Fault`, bare or wrapped, and `DependencyLockError::Input`. Such a refusal reached after a
  refuted run is `ReplayRefused`, never `Declined`, which QSL reserves for a refusal before any
  backend run, when nothing was refuted.
- The Kani adapter shall map to `Failed` exactly these failures, which this repository raises and
  which carry no QSL catalog code: `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript,
  WrongArm, Identity}`, `FrameReplayError::{Transcript, Envelope, Name}`,
  `ReplayPackageError::InvalidFunction`, a Kani playback outside the harness proof bound, and a
  decode failure (`DecodeFailure`), which is a playback that does not type against the bindings
  this repository persisted. `ReplayPackageError::InvalidFunction` wraps a discarded `InvalidIdentifier`
  from `Identifier::new`, which has no code. `FrameReplayError::Name` wraps QSL's
  `EmptyQualifiedName`, which has no code; as built, no call reaches it, because every
  `QualifiedName::new` call passes a non-empty list.
- The replay package builder shall define no duplicate-identity error of its own, so
  `DependencyLockError` has no `Duplicate` variant.
- The replay package builder shall refuse a repeated dependency identity only through QSL's
  `DependencyInput::new`, so that QSL's refusal and its code reach the map as
  `DependencyLockError::Input`. A second copy of QSL's check here would mint a CG-origin refusal for a
  condition QSL codes, and a `Failed` for it would report an input defect as a fault. A `ReplayRefusal` wrapped in `SpineReplayError::Refused` or
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
| FR-029-AC-11 | `falsified` with each CG-raised failure that carries no QSL code maps to `Failed`: `SpineReplayError::UnboundArgument`, `FieldDelimiter`, `Transcript`, `WrongArm` and `Identity`; `FrameReplayError::Transcript`, `Envelope` and `Name`; `ReplayPackageError::InvalidFunction`; a playback outside the harness proof bound; and a decode failure. No `Inconclusive(ReplayRefused)` value carries a code that no QSL refusal value supplied. | Test (TC-040) |
| FR-029-AC-12 | Across every replay settlement other than reproduced, `falsified` maps to a value other than `Refuted`. | Test (TC-040) |
| FR-029-AC-13 | `falsified` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`, each bare and wrapped in `ReplayPackageError` and `FrameReplayError`, maps to `Inconclusive(ReplayRefused)` carrying `CallSiteRefusal::code()` or `DependencyInputRefusal::code()` of that refusal, and never to `Declined`. | Test (TC-040) |
| FR-029-AC-14 | A lock that selects one library identity twice is refused by QSL's `DependencyInput::new` as `DuplicateIdentity` (`invalid_package`) and reaches the map as `DependencyLockError::Input`, so `falsified` with it maps to `Inconclusive(ReplayRefused)` carrying `invalid_package`. | Test (TC-040) |

## Dependencies

- **Upstream**: [FR-017](./FR-017-kani-execution-evidence.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md), which classify the outcome; QSL's `qsl-replay`, which
  defines `TerminalValue` and the replay result and refusal types; QSL ADR-013 O-16 and C-09, ADR-011
  T-13 and FR-121; QSpec FR-331. The `Inconclusive` terminal value and its `ReplayParity` and
  `ReplayRefused` causes are not yet in QSL.
- **Downstream**: [TC-040](../matrix/TC-040-run-outcome-terminal-record.md).

## Status

Planned (Linear IR-465). No code implements this map at this revision. The code half is blocked on
QSL types that are not merged.

Merged in QSL `main`, read at this revision: `TerminalValue` has seven variants and no
`Inconclusive`, `Declined` carries a `ProofRefusalCause` only, `InconclusiveCause` has only
`KaniVacuousProof`, and `Proved` carries a `u32`. `ReplayRefusal::code()`,
`CallSiteRefusal::code()` and `DependencyInputRefusal::code()` are public, so every code the map
carries exists. Pending in QSL (QSL-351, in progress; what has merged under it is the `ToolPin`
deletion only): `TerminalValue::Inconclusive`, `InconclusiveCause::ReplayParity` and
`ReplayRefused(Code)`, a `Declined` that carries a code, and a typed request index in the terminal
record. FR-029-AC-8, AC-9, AC-13 and AC-14 cannot be built until those types merge, and the code is
written against the merged API then.

Vacuous and cover-unsatisfied rows. QSL ruled, relayed on IR-465 (a QSL ruling recorded by the
planner), that vacuity stays `Proved { success_checks: 0 }`, reported through a separate enum, with
no `NonZero` count. The rows above and FR-029-AC-2 follow merged ADR-013 C-09 and that ruling.

Setup refusal on data. QSL ruled, relayed on IR-465 (a QSL ruling recorded by the planner), that the
settlement of a non-fault `CallSiteRefusal` depends on when it happens: after a backend refutation,
as in this repository, it is `Inconclusive(ReplayRefused(Code))` with the QSL catalog code;
`Declined` is for a refusal before any backend run, when nothing was refuted. QSL's FR-121 text,
which today says `declined` unconditionally, is being amended to say so under QSL-351. This FR
follows the ruling. `DependencyLockError::Input` is this repository applying the same timing rule:
the relayed answer names the call-site refusal only, and the extension is sound because the same
`DependencyInputRefusal` reaches the map as `CallSiteRefusal::DependencyInput` when QSL builds the
input. `ReplayPackageError::InvalidFunction` and `FrameReplayError::Name` carry no QSL code and are
CG defects, so `Failed`: `InvalidFunction` wraps a discarded `InvalidIdentifier` that has no code,
and `Name` is unreachable as built.

Repeated dependency identity. QSL ruled, relayed on IR-465 (a QSL ruling recorded by the planner),
that this repository deletes its own duplicate pre-check and builds QSL's dependency input, so that
`DependencyInput::new` refuses `DuplicateIdentity` as `invalid_package` with cause
`conflicting-definition`; it arrives as `DependencyLockError::Input` and settles by timing like
every other setup refusal. QSL rejected a second copy of QSL's check here and rejected `Failed`,
which would report an input defect as a fault. FR-029-AC-14 states this.

What is buildable now, and what is not. Merged QSL code already has `DependencyInput::new` with
`DuplicateIdentity`, so deleting the pre-check in the lock admission and the `Duplicate` variant,
and letting QSL's constructor refuse, needs no pending QSL type. That is the job of the code change,
not of this spec change; the existing test that expects the repeated identity as `Duplicate` changes
with it. The mapping of the resulting `Input` to `ReplayRefused` waits on the pending
`Inconclusive` types above.

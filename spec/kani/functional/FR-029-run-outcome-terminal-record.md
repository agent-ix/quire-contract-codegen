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
- CG defect: exactly the errors FR-029-AC-11 and FR-029-AC-16 list, which this repository raised
  and which carry no QSL catalog code, including `ReplayPackageError::InvalidFunction`,
  `FrameReplayError::Name` and the `StateClauseReplayError` variants `Name`, `Transcript`,
  `Envelope`, `Document`, `MissingField`, `UndeclaredField`, `DuplicateField`, `OutOfDomain` and
  `UnsupportedOperationShape`, and a replay that settled `ReproducedWithEvaluatedWitness` in a
  category other than `violation`. QSL proves `violation` for every replay, so no QSL result is
  that state; only this repository's public `EvidenceFailureCause::Verdict` can state it, which
  makes it a CG defect (FR-016-AC-13: never a reproduced failure);
- setup refusal on data: a refusal of the replay setup that this repository reaches after the run
  was falsified, that is not a `ReplayRefusal` returned by `qsl_replay::replay`, and that carries
  a QSL catalog code: a `CallSiteRefusal` other than `Fault` (code from `CallSiteRefusal::code()`)
  and `DependencyLockError::Input` (code from `DependencyInputRefusal::code()`), each bare or
  wrapped in `ReplayPackageError`, `FrameReplayError` or `StateClauseReplayError`. A lock that repeats a dependency identity
  is one of these: QSL's `DependencyInput::new` refuses it as `DuplicateIdentity`, code
  `invalid_package` with cause `conflicting-definition`, and it arrives as
  `DependencyLockError::Input`.

The driver runs the Kani obligation and the replay, pairs the two and writes each item's terminal
record (QSL ADR-011 T-13); this repository provides the map and its typed inputs. For the existing
source-predicate map, a replay settlement accompanies a falsified outcome only, because it replays a
counterexample; every other ordinary outcome maps with no settlement. The planned composite parity
route below instead consumes a same-claim QSL settlement for verified and falsified shadows through
its typed binding, without relabelling verified input as a counterexample settlement.

The map covers outcomes of runs only. An item that settles `unsupported`, `requires-bound` or
`invalid-request` settles at negotiation ([FR-019](../../routed/functional/FR-019-capability-settlement.md)), with a
warning naming its capability kind from `quire.capability-kind/v1` (QSpec FR-290) for
`unsupported`. It has no run, no artifact and no terminal value.

## Inputs

- One `KaniRunOutcome` ([FR-017](./FR-017-kani-execution-evidence.md),
  [FR-028](./FR-028-bounded-proof-ceilings.md)) and the SUCCESS check count its transcript reports.
- For a falsified outcome, the replay settlement of its counterexample, as the Description states
  it.
- Planned (IR-241): the proof subject (`production` or `bounded_shadow`, FR-028) and, for a
  verified outcome, its proof strength (FR-028-AC-17). A family with no shadow carries subject
  `production` and strength `production_proved`.
- Planned (IR-635): retained `refinement_failed` disagreement whatever the backend outcome,
  including the widened interim inconclusive/cover-unsatisfied refusal case in AC-21.

- Planned CG consumer (IR-666; QSL-640 delivered): the same-claim composite parity
  settlement/record, binding, bounds and refinement evidence defined below and by
  [FR-033](../../replay/functional/FR-033-composite-parity-replay-binding.md).

## Outputs

- One `qsl_replay::TerminalValue` for every pair the Inputs define.
- For the ordinary source-predicate input, a typed refusal, `TerminalPairError`, for an invalid
  pair: a falsified outcome with no replay settlement (`MissingSettlement`), or any other outcome
  with one (`UnexpectedSettlement`).
- Planned (IR-241): `TerminalPairError::NonProductionProof` carries the proof strength for a
  verified outcome whose strength is not `production_proved`; `ShadowCounterexample` refuses a
  falsified `bounded_shadow` outcome. IR-241 owns these variants and their proof-strength input.
- Planned (IR-635): the same `NonProductionProof` variant also carries `refinement_failed` for a
  `bounded_shadow` `inconclusive` or `cover-unsatisfied` outcome retaining that disagreement.
  This is the widened interim case; IR-635 does not reassign ownership of the variants.
  Neither interim refusal returns a terminal value while CG has not consumed QSL's parity facade.
- Planned (IR-635): the admitted composite route preserves QSL's terminal record/category/cause
  under AC-17 and AC-19 to AC-28; its typed binding refuses missing or another claim's settlement
  without a value.

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
  | `falsified` | reproduced in a category other than `violation` (a CG defect; no QSL result states it) | `Failed` |

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
  cover-unsatisfied run are both vacuous: the run established no property over a satisfiable domain.
- An exhausted unwind bound is a configured resource bound the run exhausted before it completed. A
  run with no verdict is a build, launcher or solver failure, so the tool failed, not the property.
- A failure without a counterexample and a missing cover summary each map to `Failed`: under the
  option vector every harness identity records, the backend prints a concrete playback for every
  failed check and a cover summary for every run, so either omission is the tool breaking its output
  contract, not a verdict on the property.
- The Kani adapter shall map a falsified outcome to `Refuted` only with a reproduced replay. A
  falsified outcome whose replay did not reproduce it is never `Refuted`. ### Composite shadow
  publication (planned CG consumer IR-666; QSL-640 delivered)

The production and source-predicate rows above retain their meaning. For a bound, node-selected
composite parity claim, the adapter shall consume QSL's same-claim settlement/record from
[FR-033](../../replay/functional/FR-033-composite-parity-replay-binding.md), rather than apply the
source-predicate `reproduced` row. The interim `NonProductionProof` and `ShadowCounterexample`
refusals remain the truthful unavailable-route disposition until CG consumes the delivered QSL
parity facade; they are then replaced for this admitted route by the rows below. A spec-only merge does not
release that implementation gate.

Before CG consumes QSL-640, a bounded-shadow `inconclusive` or `cover-unsatisfied` outcome with
retained `refinement_failed` takes the typed interim `NonProductionProof` refusal carrying that
strength and yields no terminal value. Outcomes without retained disagreement still take the
ordinary inconclusive rows. This total interim map does not weaken the delivered parity route's
disagreement-first `Failed`/CG-owned `CgDefect` rule or invent an available QSL cause.

FR-028 AC-17 retains every source proof strength. The adapter shall project the closed strength set
into the proposed QSL evidence as follows, without a wildcard or deleting a source strength:

| Source strength | QSL parity evidence |
|---|---|
| `production_proved` | ordinary production map above; never composite shadow evidence |
| `shadow_proved_refinement_exhaustive` | `Refinement::Exhausted` |
| `shadow_proved_refinement_sampled` | `Refinement::NotExhausted` |
| `shadow_proved_refinement_not_run` | `Refinement::NotExhausted` |
| `shadow_proved_refinement_inconclusive` | `Refinement::CeilingReached`, retaining the actual ceiling evidence |
| `refinement_failed` | `Refinement::Disagreed`, retaining the native disagreement |

QSL FR-358 and its public composite parity facade were merged in QSL #645. CG's consumer is still
planned for IR-666 code. QSL derives declared operand bounds and coverage
from the same recompiled node claim under FR-033; CG does not assert its own declared-bound
completeness. Common admission checks claim/source/node/occurrence/bounds/O-09 identity.
When common admission succeeds, the adapter shall apply refinement disagreement to that valid claim
before any backend outcome or replay agreement. A retained `refinement_failed` strength produces
`Failed`/CG-owned `CgDefect` whatever the shadow backend settled, including a falsified replay that
agrees on its replayed case. The
disagreement record is never lost to an early operand admission or missing/fault native-observation
path; those checks follow the same-valid-claim Disagreed row under FR-033.

For the remaining completed proof-path cases, the adapter shall preserve this first-applicable order
while keeping the original backend classification:

| Priority | Backend outcome and same-claim evidence | QSL terminal payload / record reading |
|---|---|---|
| 1 | any outcome with `refinement_failed` / `Refinement::Disagreed` | `Failed`, with CG-owned `CgDefect` cause |
| 2 | verified shadow with `CeilingReached` refinement | `Incomplete(ResourceExhausted)`, category incomplete |
| 3 | vacuous-proof `inconclusive` or `cover-unsatisfied`, zero SUCCESS checks | ordinary `Proved { success_checks: 0 }`, category inconclusive with `KaniVacuousProof` |
| 4 | verified shadow, nonzero count n, `Exhausted` and every QSL-declared bound key covered | `Proved { success_checks: n }` |
| 5 | remaining completed verified shadow with nonzero count | `Tested` |

Row 3 consumes the ordinary vacuous backend classification; it does not coerce that run into a
verified outcome or send invented `Verified` evidence to QSL. A verified backend outcome has at
least one SUCCESS check. QSL's own vacuous payload/category policy remains authoritative when
reading any returned record. The zero-count facade safeguard is not claimed as a reachable CG
verified-run row.

In the absence of a retained refinement disagreement, every `bounded_shadow` inconclusive backend
outcome takes the ordinary inconclusive rows, without a proof strength or a completed parity
settlement: vacuous proof and cover-unsatisfied give the vacuous record; timed-out gives
`Incomplete(TimedOut)`; memory-exhausted and unwind-bound-exhausted give
`Incomplete(ResourceExhausted)`; no-verdict, failure-without-counterexample and
missing-cover-summary give `Failed`. These source classes are not deleted by the parity route.

A backend run stopped at a wall-clock or memory ceiling is `inconclusive` under FR-028 AC-2/3 and
takes those ordinary rows. A verified shadow whose refinement run hits either ceiling retains
strength `shadow_proved_refinement_inconclusive` under FR-028 AC-24 and takes priority 2, retaining
which ceiling stopped it. A completed sampled comparison differs from an interrupted refinement run;
no ceiling result becomes `NotExhausted` or `Tested`. QSL settlement's own resource/admission
refusal retains its typed refusal reading rather than entering priority 5. The backend
classification, refinement evidence and QSL terminal record are separate stages.

For an identity-valid falsified composite claim, both paths share the same `Refinement` enum. The
adapter shall preserve QSL FR-358's first-match F-1 to F-7 rows through FR-033: Disagreed gives
Failed/CgDefect; native Incomplete/ExecutionFault gives GeneratedFault/Failed retaining native and
NativeCause before operand admission; RefusedInput gives Inconclusive(ReplayRefused) with its actual
QSL code and operand index; an admission accounting limit gives Incomplete(ResourceExhausted) with
Admission stage (QSL owns skipped exact evaluation); an exact limit gives Incomplete(ResourceExhausted) with
ExactEvaluation stage; refinement ceiling gives Incomplete(ResourceExhausted) with
RefinementCeiling stage; otherwise exact-versus-shadow
verdict/pair-count divergence gives Failed/CgDefect and agreement gives Inconclusive(ScalarAgrees)
with the CompositeEquality claim/outcome. Completed/Refused native remains evidence, not the
settlement oracle. No row gives Refuted. CG's own source/package, harness, playback and content
prechecks, an absent report, or a failed full `CompositeIdentity` binding under FR-033 return a
typed refusal with no terminal value. A binding-valid QSL
`CompositeParityResult::Refused` report does carry a terminal value:
non-fault `ReplayRefusal` yields Inconclusive(ReplayRefused) with QSL's code, while
`ReplayRefusal::Fault` and `ReplayRefusal::Admission(AdmissionFailure::Fault)` yield Failed.
QSL `prepare` refusals precede Disagreed; operand-admission and exact-comparison refusals follow
F-1, so Disagreed wins over those later faults. CG consumes only reports actually returned by QSL's
public facade and never constructs an invariant-fault report.
All owning API/cause representations remain gated on CG's IR-666 consumer implementation; no
predicate cause is fabricated.

The adapter shall read the actual QSL record category and cause, including inconclusive `Proved {
success_checks: 0 }`, without promoting a payload into a success record. AC-17 and AC-19 to AC-28
own this map; FR-028 AC-17/24 retain refinement and ceiling ownership. The ordinary
`ReplaySettlement` input and its MissingSettlement/UnexpectedSettlement rules remain scoped to
source-predicate replay. The composite route requires a distinct typed same-claim settlement input;
AC-15 cannot reject that input as an ordinary unexpected replay settlement.

- Planned (pending QSL-634, IR-460): the Kani adapter shall read a post-state value outside its
  field's declared range as the subject's output and so as the witness, not as a refused input: QSL
  ruled (QSL-634, filed, not merged) that inputs are refused and outputs are evidence, so an
  out-of-range pre state or argument stays refused (`invalid_runtime_input`), and QSL will admit an
  out-of-range post state as an exact observation, never clamped, and settle the replay as
  reproduced or violated naming the field, the range and the observed value. The map then reads it
  as any settled replay: a reproduced `violation` is `Refuted`, not `Inconclusive(ReplayRefused)`
  (FR-029-AC-18). Until QSL-634 lands this repository keeps today's behaviour, and builds nothing
  against it.
- The Kani adapter shall classify a fault by walking the whole error the replay path returned, not
  by its top variant. A fault is `ReplayRefusal::Fault`, `ReplayRefusal::Admission` carrying
  `AdmissionFailure::Fault`, and `CallSiteRefusal::Fault`, which reaches the map directly and
  wrapped in `ReplayPackageError::CallSite`, in `FrameReplayError::CallSite` and in
  `StateClauseReplayError::CallSite`. Each maps to `Failed`, and so does a `ReplayRefusal` fault
  wrapped in `SpineReplayError::Refused`, `FrameReplayError::Refused` or
  `StateClauseReplayError::Refused` (FR-029-AC-10).
- The Kani adapter shall put only a code from QSL's closed catalog inside
  `Inconclusive(InconclusiveCause::ReplayRefused)`, and shall define no code of its own for it.
- The Kani adapter shall map a failed replay setup to `Inconclusive(ReplayRefused)` carrying the QSL
  catalog code only when a QSL refusal value supplies that code: a `CallSiteRefusal` other than
  `Fault`, bare or wrapped, and `DependencyLockError::Input`. Such a refusal reached after a refuted
  run is `ReplayRefused`, never `Declined`, which QSL reserves for a refusal before any backend run,
  when nothing was refuted.
- The Kani adapter shall map to `Failed` exactly these failures, which this repository raises and
  which carry no QSL catalog code: `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript,
  WrongArm, Identity}`, `FrameReplayError::{Transcript, Envelope, Name, NotAFrame, FieldSetMismatch,
  Decode, OutOfDomain, PreState, ScopeMismatch, Identity}`, `ReplayPackageError::InvalidFunction`, a
  Kani playback outside the harness proof bound, a decode failure (`DecodeFailure`), which is a
  playback that does not type against the bindings this repository persisted, and a replay
  reproduced in a category other than `violation`. `ReplayPackageError::InvalidFunction` wraps a
  discarded `InvalidIdentifier` from `Identifier::new`, which has no code. `FrameReplayError::Name`
  wraps QSL's `EmptyQualifiedName`, which has no code; as built, no call reaches it, because every
  `QualifiedName::new` call passes a non-empty list.
- The Kani adapter shall read the result and errors of the state-clause replay path (FR-024) as
  FR-029-AC-16 states, by the same rules as the frame path's `FrameReplayError`, and shall map
  `StateClauseReplayError::{Name, Transcript, Envelope, Document, MissingField, UndeclaredField,
  DuplicateField, OutOfDomain, UnsupportedOperationShape}` to `Failed`.
- The Kani adapter shall read a `ReplayRefusal` wrapped in `SpineReplayError::Refused` or
  `FrameReplayError::Refused` as the refused or fault reading by its walked content, not as a CG
  defect. [AD-001](../../assurance/AD-001-codegen-architecture.md)'s failure view keeps each a
  distinct typed state before the map.
- The Kani adapter shall expose no `proof_category` function. A value's category is
  `TerminalValue::category()` of it and nothing else
  ([AD-003](../../assurance/AD-003-evidence-chain.md) E-9).
- The Kani adapter shall refuse a falsified outcome given no replay settlement, and any other
  outcome given one, with a `TerminalPairError`, and shall return no terminal value for it. This is
  a design choice: the map takes the outcome and an optional settlement, so the driver calls one
  function with one shape, and the pairing the Inputs state is checked at that call. The refusal is
  typed rather than a value, and FR-029-AC-15 asserts it. Other designs make every pair total
  without a refusal, for example a replay callback the map calls only for a falsified outcome; they
  were not chosen.
- The ordinary production/source-predicate map shall map no outcome to `Tested`; only an admitted
  QSL composite settlement may return `Tested` under AC-19.
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
| FR-029-AC-6 | The ordinary production/source-predicate map returns no `Tested`. | Test |
| FR-029-AC-8 | `falsified` with a replay disagreement of each `DisagreementCause` (`Verdicts`, `Witness` and `NoValue`) maps to `Inconclusive(ReplayParity)` carrying that `DisagreementCause`. | Test (TC-040) |
| FR-029-AC-9 | `falsified` with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying `ReplayRefusal::code()` of that refusal. | Test (TC-040) |
| FR-029-AC-10 | `falsified` with a fault maps to `Failed` for each of: the call-site fault's own reading (`ReplaySettlement::Fault`, which QSL's replay result does not carry: QSL reports a fault as an error); `ReplayRefusal::Fault` and `ReplayRefusal::Admission(AdmissionFailure::Fault)`, each bare and as the cause of `SpineReplayError::Refused`, `FrameReplayError::Refused` and `StateClauseReplayError::Refused`; and `CallSiteRefusal::Fault` bare and wrapped in `ReplayPackageError::CallSite`, `FrameReplayError::CallSite` and `StateClauseReplayError::CallSite`. | Test (TC-040) |
| FR-029-AC-11 | `falsified` with each CG-raised failure that carries no QSL code maps to `Failed`: `SpineReplayError::UnboundArgument`, `FieldDelimiter`, `Transcript`, `WrongArm` and `Identity`; `FrameReplayError::Transcript`, `Envelope`, `Name`, `NotAFrame`, `FieldSetMismatch`, `Decode`, `OutOfDomain`, `PreState` (a pre state the generator cannot read from the driver's documents, or one that differs from the playback; a document whose bytes do not match its digest is QSL's `Refused`, read by its code), `ScopeMismatch` and `Identity`; `ReplayPackageError::InvalidFunction`; a playback outside the harness proof bound; a decode failure; and a replay reproduced in a category other than `violation`. No `Inconclusive(ReplayRefused)` value carries a code that no QSL refusal value supplied. | Test (TC-040) |
| FR-029-AC-12 | Across every replay settlement other than reproduced, `falsified` maps to a value other than `Refuted`. | Test (TC-040) |
| FR-029-AC-13 | `falsified` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`, each bare and wrapped in `ReplayPackageError` and `FrameReplayError`, maps to `Inconclusive(ReplayRefused)` carrying `CallSiteRefusal::code()` or `DependencyInputRefusal::code()` of that refusal, and never to `Declined`. | Test (TC-040) |
| FR-029-AC-14 | `falsified` with a `DependencyLockError::Input` that carries QSL's `DuplicateIdentity` refusal (code `invalid_package`), as a lock whose only defect is a repeated library identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)` carrying `invalid_package`. | Test (TC-040) |
| FR-029-AC-15 | For the ordinary source-predicate `ReplaySettlement` input, a falsified outcome given no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other outcome (`verified`, `cover-unsatisfied` and every inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`; neither returns a terminal value. The distinct typed composite-parity settlement input is governed by AC-17 and AC-19 to AC-28, not this criterion. | Test |
| FR-029-AC-16 | The state-clause replay path (FR-024) settles as the other replay paths do: a `StateClauseReplayResult` that settles `ReproducedWithEvaluatedWitness` in category `violation` is a reproduction and an `inconclusive` one is a disagreement carrying its `DisagreementCause`, so `falsified` maps to `Refuted` and `Inconclusive(ReplayParity)`; `StateClauseReplayError::Refused` and `CallSite` read as `ReplayRefusal` and `CallSiteRefusal` do for a non-fault refusal (`Inconclusive(ReplayRefused)` with its catalog code; their fault reading is FR-029-AC-10's), and `Dependencies` as `DependencyLockError` does; `Name`, `Transcript`, `Envelope`, `Document`, `MissingField`, `UndeclaredField`, `DuplicateField`, `OutOfDomain` and `UnsupportedOperationShape` carry no QSL code and each maps to `Failed`. A missing state field in CG's own harness playback and a value outside the proof bound are CG defects, and an operation shape CG does not support is a CG limit, not a QSL data refusal; none maps to `Incomplete` or to `Inconclusive(ReplayRefused)`. | Test (TC-040) |
| FR-029-AC-17 | PLANNED (IR-635), GATED until CG consumes the delivered QSL parity facade. The strength projection enumerates every FR-028-AC-17 strength: production retains its ordinary map; exhaustive projects to Exhausted, sampled/not_run to NotExhausted, ceiling-inconclusive to CeilingReached, and refinement_failed to Disagreed. Backend-inconclusive outcomes acquire no invented verified proof strength. | Test |
| FR-029-AC-18 | For the `falsified` state-clause run of the wrapping debit subject `deposit_debiting`, whose post-state value (-1) lies outside `balance`'s declared range, this repository builds the post snapshot with that exact unclamped value and does not refuse it itself (not as `StateClauseReplayError::OutOfDomain` or any other CG defect), and the run maps to `Refuted`, the ordinary violated terminal, and not to `Inconclusive(ReplayRefused)` or `Failed`. How QSL settles the replay of such a snapshot is QSL-634's to decide; this criterion states CG's own obligation and the target terminal. PLANNED, pending QSL-634 (IR-460), and nothing is built against it: QSL admission refuses such a post snapshot today, so the run reads `Inconclusive(ReplayRefused(InvalidRuntimeInput))` until QSL-634 lands. | Test (TC-040) |
| FR-029-AC-19 | PLANNED/GATED (IR-635/QSL-640). After the cross-outcome disagreement rule, verified shadow CeilingReached yields Incomplete(ResourceExhausted); nonzero Exhausted with QSL-derived complete coverage yields Proved n; remaining completed nonzero verification yields Tested. Tested is excluded for disagreement, ceilings and zero checks; tightening or missing coverage cannot promote it to Proved. | Test |
| FR-029-AC-20 | PLANNED CG CONSUMER (IR-666; QSL FR-358 delivered). An identity-valid falsified composite shadow consumes the shared Refinement and ordered F-1 to F-7 result: Disagreed/Failed/CgDefect; native Incomplete or ExecutionFault as GeneratedFault/Failed with NativeCause, winning over an invalid operand and every later limit; RefusedInput/ReplayRefused with QSL code and operand index, winning over CeilingReached; admission accounting exhaustion as Incomplete(ResourceExhausted)/Admission with counter; exact-evaluation exhaustion as Incomplete(ResourceExhausted)/ExactEvaluation before CeilingReached; CeilingReached as Incomplete(ResourceExhausted)/RefinementCeiling; finally exact-versus-shadow divergence Failed/CgDefect or Inconclusive(ScalarAgrees) with CompositeEquality claim/outcome. CG asserts typed report/terminal evidence; QSL FR-358 owns skipped internal work on F-2 and F-4. No row returns Refuted; Completed/Refused native is evidence only. | Test |
| FR-029-AC-21 | PLANNED: IR-241 owns the refusal variants and verified proof-strength input; IR-635 owns the widened inconclusive/cover-unsatisfied case. Until CG consumes the delivered QSL parity facade, a verified nonproduction shadow takes NonProductionProof and a falsified shadow takes ShadowCounterexample; inconclusive and cover-unsatisfied shadows with retained refinement_failed take NonProductionProof carrying that strength. All are typed interim refusals with no terminal value, never a fabricated predicate replay or terminal cause. | Test |
| FR-029-AC-22 | PLANNED (IR-635). The strength set enumerated by the map equals FR-028-AC-17's closed set; adding a strength without its projection and outcome row fails the build or test rather than entering a wildcard mapping. | Test |
| FR-029-AC-23 | PLANNED/GATED (IR-635/QSL-640). A stopped backend run retains its ordinary timeout/memory inconclusive classification; a stopped refinement retains FR-028-AC-24's CeilingReached projection and recorded ceiling. Neither stop, nor QSL settlement resource refusal, becomes completed NotExhausted evidence or Tested. | Test |
| FR-029-AC-24 | PLANNED/GATED (IR-635/QSL-640). After common claim/source/node/bounds/O-09 validation, retained refinement_failed/Disagreed evidence for the same valid claim yields Failed/CgDefect for verified, falsified, inconclusive and cover-unsatisfied backend outcomes, including zero checks, an operand field x declared Int[0, 9] but supplied as x: 12, missing/fault native observation and a falsified replay agreeing on its retained case; no early operand/native setup refusal loses this disagreement, which precedes resource, vacuity and agreement rows. Missing observations are never fabricated. | Test |
| FR-029-AC-25 | PLANNED (IR-635). With no retained refinement disagreement, every bounded_shadow inconclusive reason maps through the ordinary rows: vacuous proof and cover-unsatisfied are Proved0; timeout is Incomplete(TimedOut); memory and unwind exhaustion are Incomplete(ResourceExhausted); no-verdict, failure-without-counterexample and missing-summary are Failed. No vacuous outcome is coerced into verified evidence. | Test |
| FR-029-AC-26 | PLANNED/GATED (IR-635/QSL-640). The converter preserves the actual QSL record category/cause: Proved0 remains inconclusive/KaniVacuousProof and is never promoted by its payload. Zero-count backend cases use AC-25, not an unreachable verified-count-zero row. | Test |
| FR-029-AC-27 | PLANNED/GATED (IR-635/QSL-640). The distinct typed composite settlement input refuses a missing settlement or another claim's node/run/operation/operand/domain/limits/content binding with no value; ordinary source-predicate AC-15 does not consume or reject a valid verified-parity settlement as UnexpectedSettlement. | Test |
| FR-029-AC-28 | PLANNED CG CONSUMER (IR-666; QSL FR-358 delivered). For any full-identity-bound QSL `CompositeParityResult::Refused` report, CG preserves `report.terminal_value()`: non-fault refusal is Inconclusive(ReplayRefused) with QSL's code, while `Fault` and `Admission(Fault)` are Failed. QSL `prepare` refusal precedes F-1; operand-admission and exact-comparison refusals follow F-1, so Disagreed wins over those later faults. CG verifies a reachable non-fault report by invoking QSL's public facade with a valid wire request and a replay input-byte limit below that request's size, then passing the actual report through its public converter. It inspects the public QSL terminal mapping and CG pass-through for invariant fault cases; it neither fabricates a report nor claims a public input that induces an invariant fault. | Inspection |

### Mutation FR-029-AC-17 detects

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-029-AC-17 | Add a proof strength to FR-028-AC-17 without a row here, or map it through a wildcard arm to `Proved` or `Failed`, so a verified shadow harness whose refinement hit a ceiling has no defined output or reads as proved. |

## Implementation Gate

QSL-640 delivered the node-selected parity facade in QSL #645, including F-1 to F-7 and
`IncompleteStage::Admission`. CG's consumer remains gated on IR-666 code and the IR-665 gate
repair. [FR-033](../../replay/functional/FR-033-composite-parity-replay-binding.md) owns the
same-claim binding and the CG-owned CgDefect attachment before interim refusals are replaced.

## Dependencies

- **Upstream**: [FR-017](./FR-017-kani-execution-evidence.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md), which classify the outcome; QSL's `qsl-replay`, which
  defines `TerminalValue` and the replay result and refusal types; QSL ADR-013 O-16 and C-09, ADR-011
  T-13 and FR-121; QSpec FR-331. The `Inconclusive` terminal value and its `ReplayParity` and
  `ReplayRefused` causes are merged in QSL `main`.
- **Downstream**: [TC-040](../matrix/TC-040-run-outcome-terminal-record.md).

## Status

Built (Linear IR-465) in `kani/terminal.rs` as `run_terminal_value`, with the typed
`ReplaySettlement` it reads and, in `replay/`, the `From` conversions from every error the replay
path returns, and for the state-clause path (FR-029-AC-16, IR-460). Every built criterion is backed
by a tagged test except one. FR-029-AC-3 is backed for the timed-out and exhausted-unwind-bound
reasons only: `KaniInconclusiveReason` has no memory-exhausted reason until FR-028-AC-3 adds one,
and the map's `match` fails to compile there until that arm is written. FR-029-AC-10 is backed:
`tc_040_a_fault_in_any_replay_wrapper_is_failed` builds each fault wrapper the criterion names from
QSL's constructible `InternalFault` (re-exported by `qsl-replay`, `InternalFault::new`, QSL `main`
bcca433, QSL #635) and asserts `Failed` through the whole error. The fault readings of
`StateClauseReplayError::Refused` and `CallSite` are listed under FR-029-AC-10, so FR-029-AC-16
holds only clauses a test asserts. IR-241 owns the planned proof-subject/verified-strength inputs
and interim NonProductionProof/ShadowCounterexample variants; IR-635 owns their widened
inconclusive/cover-unsatisfied disagreement case and the parity route. FR-029-AC-17 and AC-19 to
AC-27 (IR-635, CG consumer gate for delivered QSL-640), AC-28 (IR-666) and FR-029-AC-18
(QSL-634) are planned and unbuilt.

Post state outside its range (pending QSL-634, IR-460). QSL ruled (cited as QSL-634, filed, not
merged) that inputs are refused and outputs are evidence. An out-of-range pre state or argument
stays refused (`invalid_runtime_input`). An out-of-range post-state value is the subject's output
and so the witness: QSL will admit it as an exact out-of-range observation, never clamped, and
replay will settle reproduced or violated naming the field, the range and the observed value. CG
will map it to its ordinary violated terminal, not to `Inconclusive(ReplayRefused)`. Until QSL-634
lands, CG keeps today's behaviour: QSL admission refuses an out-of-range post snapshot and the run
reads `Inconclusive(ReplayRefused(InvalidRuntimeInput))`. FR-029-AC-18 states the target and is
planned; nothing is built against it.

Two points the map decides, now stated in the Description, Outputs, Behavior and criteria above
(FR-029-AC-11 and FR-029-AC-15). A pair the driver mis-builds is a typed `TerminalPairError`, not a
value. That is a design choice, not a necessity: the map takes the outcome and an optional
settlement so the driver calls one function with one shape, and the typed refusal is asserted by
FR-029-AC-15. A replay that settled `ReproducedWithEvaluatedWitness` in
a category other than `violation` carries no `DisagreementCause` and is not a refutation
(FR-016-AC-13); the conversion reads it as a CG defect, `Failed`, rather than inventing a cause.

Merged in QSL `main`, read at this revision: `TerminalValue::Inconclusive`,
`InconclusiveCause::{ReplayParity, ReplayRefused(Code)}`, `Proved { success_checks: u32 }` read as
vacuous at zero through `ReportedInconclusiveCause`, `Declined { cause, code: DeclineCode }`, and
`TerminalValue::from_replay_refusal`, which the map uses for a refusal `qsl_replay::replay`
returned. `DeclineCode` has a QSL catalog arm, `Qsl(Code)`, and a STD-001 arm, `Std001(Std001Code)`
(QSL #634); this map produces no `Declined`, which only [FR-030](./FR-030-ir-outcome-terminal-map.md)'s
map does.

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
`conflicting-definition`; it arrives as `DependencyLockError::Input` and settles by timing: QSL
settles `Declined` for a refusal before a backend run and `ReplayRefused` after a refutation. This
repository builds the lock only after Kani refuted, so for it the settlement is always
`ReplayRefused`. QSL rejected a second copy of QSL's check here and rejected `Failed`, which would
report an input defect as a fault. FR-016-AC-24 states the builder's behaviour, and FR-029-AC-14
the mapping.

The pre-check in the lock admission and the `Duplicate` variant are deleted, and QSL's
`DependencyInput::new` refuses the repeated identity (FR-016-AC-24, asserted by
`tc_026_a_lock_repeating_a_dependency_is_refused`); the resulting `Input` maps to `ReplayRefused`
(FR-029-AC-14).

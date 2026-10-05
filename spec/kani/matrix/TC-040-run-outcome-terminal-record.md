---
id: TC-040
title: "Verify the total map from a Kani run outcome to QSL's terminal value"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: verifies
---
# TC-040: Verify the total map from a Kani run outcome to QSL's terminal value

## Description

Verify that each pair of run outcome and replay settlement maps to the QSL terminal value FR-029's
tables state, and that a falsified run is `Refuted` only with a reproduced replay.

## Test Procedure

1. Map `verified` with three SUCCESS checks, and `falsified` with a reproduced replay.
2. Map `inconclusive` with the vacuous-proof reason, and `cover-unsatisfied`. Read each value's QSL
   category and vacuity cause.
3. Map `inconclusive` with the timed-out, memory-exhausted and exhausted-unwind-bound reasons.
4. Map `inconclusive` with the no-verdict reason.
5. Map `inconclusive` with the failure-without-counterexample and missing-cover-summary reasons.
6. Map every outcome and reason, and collect the values.
7. Map `falsified` with a replay disagreement of each `DisagreementCause` (`Verdicts`, `Witness`,
   `NoValue`).
8. Map `falsified` with a non-fault `ReplayRefusal`.
9. Map `falsified` with a fault in each position, built from QSL's `InternalFault`: the
   call-site fault's own reading (`ReplaySettlement::Fault`; QSL's replay result carries no fault); `ReplayRefusal::Fault` and
   `ReplayRefusal::Admission(AdmissionFailure::Fault)`, each bare and inside
   `SpineReplayError::Refused`, `FrameReplayError::Refused` and `StateClauseReplayError::Refused`;
   and `CallSiteRefusal::Fault` bare and inside `ReplayPackageError::CallSite`,
   `FrameReplayError::CallSite` and `StateClauseReplayError::CallSite`.
10. Map `falsified` with each CG-raised failure FR-029-AC-11 lists, including a replay reproduced
    in a category other than `violation`, and inspect every
    `ReplayRefused` value the run produced.
11. Map `falsified` under every replay settlement other than reproduced.
12. Map `falsified` with a non-fault `CallSiteRefusal` and with a `DependencyLockError::Input`,
    each bare and wrapped in `ReplayPackageError` and `FrameReplayError`.
13. Map `falsified` with the refusal QSL's `DependencyInput::new` returns for a lock whose only
    defect is one library identity selected twice.
14. Map `falsified` with no replay settlement, and map each other outcome and reason with one.
15. Map `falsified` with each `StateClauseReplayError` variant and with a reproduced and an
    inconclusive `StateClauseReplayResult` (FR-024).
16. Map `verified` with each proof strength, and `falsified` and `inconclusive` of a
    `bounded_shadow` harness (FR-029-AC-17, planned, IR-241; see Expected Results).
17. Map the `falsified` state-clause run of `deposit_debiting`, whose post-state value lies outside
    its field's declared range, with the exact unclamped post snapshot (FR-029-AC-18, planned,
    pending QSL-634, IR-460).

## Expected Results

1. `Proved { success_checks: 3 }` and `Refuted` (FR-029-AC-1).
2. Both are `Proved { success_checks: 0 }`, category `inconclusive`, cause `KaniVacuousProof`
   (FR-029-AC-2).
3. `Incomplete(TimedOut)`, `Incomplete(ResourceExhausted)` and `Incomplete(ResourceExhausted)`
   (FR-029-AC-3).
4. `Failed` (FR-029-AC-4).
5. Each is `Failed` (FR-029-AC-5).
6. No value is `Tested` (FR-029-AC-6).
7. Each is `Inconclusive(ReplayParity)` carrying its `DisagreementCause` (FR-029-AC-8).
8. `Inconclusive(ReplayRefused)` carrying the refusal's catalog code (FR-029-AC-9).
9. Each is `Failed` (FR-029-AC-10).
10. Each CG-raised failure, including `ReplayPackageError::InvalidFunction`,
    `FrameReplayError::Name` and a replay reproduced in a category other than `violation`, is
    `Failed`, and every `ReplayRefused` value carries a code a QSL
    refusal value supplied (FR-029-AC-11).
11. No value is `Refuted` (FR-029-AC-12).
12. Each is `Inconclusive(ReplayRefused)` carrying `CallSiteRefusal::code()` or
    `DependencyInputRefusal::code()` of the refusal, and none is `Declined` (FR-029-AC-13).
13. `Inconclusive(ReplayRefused)` carrying `invalid_package` (FR-029-AC-14).
14. `TerminalPairError::MissingSettlement` and `TerminalPairError::UnexpectedSettlement`, with no
    value (FR-029-AC-15).
15. The reproduced result is `Refuted`, the inconclusive result is `Inconclusive(ReplayParity)`
    carrying its cause, `Refused` and `CallSite` read as their QSL refusals do, and `Name`,
    `Transcript`, `Envelope`, `Document`, `MissingField`, `UndeclaredField`, `DuplicateField`,
    `OutOfDomain` and
    `UnsupportedOperationShape` are `Failed`, none `Incomplete` and none
    `Inconclusive(ReplayRefused)`, and a non-fault `CallSiteRefusal` or
    `DependencyLockError::Input` wrapped in `StateClauseReplayError` is
    `Inconclusive(ReplayRefused)` with its code (FR-029-AC-16).
16. Enumerate every proof strength FR-028-AC-17 names, assert the set equals the strength table's
    rows, and map `verified` with each (`production_proved`, the four `shadow_proved_*` strengths
    `exhaustive`, `sampled`, `not_run` and `inconclusive`, and `refinement_failed`); map `falsified` of a `bounded_shadow` harness and
    `inconclusive` of a `bounded_shadow` harness (FR-029-AC-17, planned, IR-241): `Proved` for
    `production_proved` only, `NonProductionProof` carrying the strength for each other, `ShadowCounterexample`
    for the falsified shadow, with no value for any of them, and the usual value for the inconclusive one.
17. The post snapshot holds the exact unclamped value and CG refuses nothing itself, and the run is
    `Refuted`, never `Inconclusive(ReplayRefused)` or `Failed` (FR-029-AC-18, planned, pending
    QSL-634, IR-460). Not built: QSL-634 is not merged, and until it lands the run reads
    `Inconclusive(ReplayRefused(InvalidRuntimeInput))`.

## Status

Partly covered. Steps 1, 2 and 4 to 15 are tests of `tests/it/terminal_map.rs`. Step 3
asserts the timed-out and exhausted-unwind-bound reasons only, because no memory-exhausted reason
exists until FR-028-AC-3 adds it, so FR-029-AC-3 stays planned. Step 9 (FR-029-AC-10) is covered
by `tc_040_a_fault_in_any_replay_wrapper_is_failed`, which builds each fault wrapper from QSL's
constructible `InternalFault` (QSL bcca433) and asserts `Failed`. Step 15 (FR-029-AC-16, IR-460)
is covered: the reproduced and the inconclusive results are real `StateClauseReplayResult`s of the
QSL twin. The state-clause fault readings are step 9's (FR-029-AC-10). Steps 16 and 17 are planned
(FR-029-AC-17, IR-241; FR-029-AC-18, pending QSL-634) and have no test.

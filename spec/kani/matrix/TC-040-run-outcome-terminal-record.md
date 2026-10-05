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
9. Map `falsified` with a fault in each position: `ReplayRefusal::Fault`,
   `ReplayRefusal::Admission(AdmissionFailure::Fault)`, `CallSiteRefusal::Fault`, and
   `CallSiteRefusal::Fault` inside `ReplayPackageError::CallSite` and `FrameReplayError::CallSite`.
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
    `Transcript`, `Envelope`, `Document`, `MissingField`, `OutOfDomain` and
    `UnsupportedOperationShape` are `Failed`, none `Incomplete` and none
    `Inconclusive(ReplayRefused)`, and a non-fault `CallSiteRefusal` or
    `DependencyLockError::Input` wrapped in `StateClauseReplayError` is
    `Inconclusive(ReplayRefused)` with its code (FR-029-AC-16).

## Status

Partly covered. Steps 1, 2, 4 to 8 and 10 to 15 are tests of `tests/it/terminal_map.rs`. Step 3
asserts the timed-out and exhausted-unwind-bound reasons only, because no memory-exhausted reason
exists until FR-028-AC-3 adds it. Step 9 is not tested: it needs a QSL `InternalFault`, which
`qsl-replay` does not re-export, so a fault value cannot be built in this repository. FR-029-AC-3
and FR-029-AC-10 stay planned for those two reasons. Step 15 (FR-029-AC-16, IR-460) is covered:
the reproduced and the inconclusive results are real `StateClauseReplayResult`s of the QSL twin,
and no test builds a fault `Refused` for the reason step 9 gives.

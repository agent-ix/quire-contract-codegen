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
10. Map `falsified` with each CG-raised failure FR-029-AC-11 lists and inspect every
    `ReplayRefused` value the run produced.
11. Map `falsified` under every replay settlement other than reproduced.
12. HELD: map `falsified` with a setup refusal on data.

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
10. Each CG-raised failure is `Failed`, and every `ReplayRefused` value carries a code of QSL's
    `ReplayRefusal` set (FR-029-AC-11).
11. No value is `Refuted` (FR-029-AC-12).
12. HELD on a QSL or owner ruling (FR-029-AC-13).

## Status

Planned. No outcome maps to QSL's terminal value at this revision. Steps 7 and 8 wait on the unmerged QSL
`Inconclusive` terminal value, and step 12 is held on a QSL or owner ruling (FR-029 Status).

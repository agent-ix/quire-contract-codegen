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

Verify that each run outcome maps to the QSL terminal value FR-029's table states, and that the map
is one match with no wildcard arm.

## Test Procedure

1. Map `verified` with three SUCCESS checks, and `falsified`.
2. Map `inconclusive` with the vacuous-proof reason, and `cover-unsatisfied`. Read each value's QSL
   category and vacuity cause.
3. Map `inconclusive` with the timed-out, memory-exhausted and exhausted-unwind-bound reasons.
4. Map `inconclusive` with the no-verdict reason.
5. Map `inconclusive` with the failure-without-counterexample and missing-cover-summary reasons.
6. Map every outcome and reason, and collect the values.

## Expected Results

1. `Proved { success_checks: 3 }` and `Refuted` (FR-029-AC-1).
2. Both are `Proved { success_checks: 0 }`, category `inconclusive`, cause `KaniVacuousProof`
   (FR-029-AC-2).
3. `Incomplete(TimedOut)`, `Incomplete(ResourceExhausted)` and `Incomplete(ResourceExhausted)`
   (FR-029-AC-3).
4. `Failed` (FR-029-AC-4).
5. Each is `Failed` (FR-029-AC-5).
6. No value is `Tested` (FR-029-AC-6).

## Status

Implemented in `src/kani_terminal.rs`, tests `tc_040_*`, for steps 1, 2, 4, 5 and 6 and the
timed-out and exhausted-unwind-bound reasons of step 3. The memory-exhausted reason of step 3 is not
produced by any run at this revision, so that part is not tested.

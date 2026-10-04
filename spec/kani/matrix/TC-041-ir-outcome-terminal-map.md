---
id: TC-041
title: "Verify the total map from a Contract IR Kani outcome to QSL's terminal value"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: verifies
---
# TC-041: Verify the total map from a Contract IR Kani outcome to QSL's terminal value

## Description

Verify that each pair of Contract IR `KaniOutcome` and replay settlement maps to the QSL terminal
value FR-030's table states, that refusal kinds survive the map, that a `Counterexample` is
`Refuted` only with a reproduced replay, that the `Unavailable` cause code selects the unavailability
cause, and that the map is one match with no wildcard arm.

## Test Procedure

1. Map an outcome of every `KaniOutcomeKind` and collect the values.
2. Map `Refused`, `InvalidInput` and `IncompleteInput`.
3. Map `TimedOut`, `ResourceExhausted` and `Cancelled`.
4. Map `Proved` with a transcript count of three SUCCESS checks and with a count of zero, and map
   `Counterexample` with a reproduced replay.
5. Map `Inconclusive` with cause `kani_vacuous_proof`, and with another cause.
6. Map `Unavailable` with cause `kani_solver_absent`, with `kani_backend_absent`, and with another
   cause.
7. Inspect the map's source for a wildcard arm (inspection step, FR-030-AC-7).
8. Map `Counterexample` with a replay disagreement, and with a non-fault `ReplayRefusal`.
9. Map `Counterexample` with a fault in each position FR-029-AC-10 lists, and with each CG-raised
   failure FR-029-AC-11 lists.
10. Map `Counterexample` under every replay settlement other than reproduced.
11. HELD: map `Counterexample` with a setup refusal on data (a non-fault `CallSiteRefusal`,
    `DependencyLockError`, `ReplayPackageError::InvalidFunction`, `FrameReplayError::Name`).

## Expected Results

1. Exactly one value per expressible pair outside the held setup-refusal class (step 11), and none
   is `Tested` (FR-030-AC-1, FR-030-AC-6).
2. `Declined` with three distinct causes (FR-030-AC-2).
3. `Incomplete` with three distinct causes (FR-030-AC-3).
4. `Proved { success_checks: 3 }`, `Proved { success_checks: 0 }` and `Refuted` (FR-030-AC-4).
5. `Proved { success_checks: 0 }` and `Failed` (FR-030-AC-5).
6. `Unsupported(SolverAbsent)`, `Unsupported(BackendAbsent)` and `Unsupported(BackendAbsent)`
   (FR-030-AC-8).
7. The `match` over the pair has no wildcard arm (FR-030-AC-7).
8. `Inconclusive(ReplayParity)`, and `Inconclusive(ReplayRefused)` carrying the refusal's catalog
   code (FR-030-AC-9).
9. Each is `Failed` (FR-030-AC-10).
10. No value is `Refuted` (FR-030-AC-11).
11. HELD on a QSL or owner ruling (FR-030-AC-12).

## Status

Planned. No outcome maps to QSL's terminal value at this revision. Step 8 waits on the unmerged QSL
`Inconclusive` terminal value, and step 11 is held on a QSL or owner ruling (FR-029 Status).

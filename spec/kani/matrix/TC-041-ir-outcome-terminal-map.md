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

Verify that each Contract IR `KaniOutcome` maps to the QSL terminal value FR-030's table states, that
refusal kinds survive the map, that the `Unavailable` cause code selects the unavailability cause,
and that the map is one match with no wildcard arm.

## Test Procedure

1. Map an outcome of every `KaniOutcomeKind` and collect the values.
2. Map `Refused`, `InvalidInput` and `IncompleteInput`.
3. Map `TimedOut`, `ResourceExhausted` and `Cancelled`.
4. Map `Proved` with a transcript count of three SUCCESS checks and with a count of zero, and map
   `Counterexample`.
5. Map `Inconclusive` with cause `kani_vacuous_proof`, and with another cause.
6. Map `Unavailable` with cause `kani_solver_absent`, with `kani_backend_absent`, and with another
   cause.
7. Inspect the map's source for a wildcard arm (inspection step, FR-030-AC-7).

## Expected Results

1. Exactly one value per outcome, and none is `Tested` (FR-030-AC-1, FR-030-AC-6).
2. `Declined` with three distinct causes (FR-030-AC-2).
3. `Incomplete` with three distinct causes (FR-030-AC-3).
4. `Proved { success_checks: 3 }`, `Proved { success_checks: 0 }` and `Refuted` (FR-030-AC-4).
5. `Proved { success_checks: 0 }` and `Failed` (FR-030-AC-5).
6. `Unsupported(SolverAbsent)`, `Unsupported(BackendAbsent)` and `Unsupported(BackendAbsent)`
   (FR-030-AC-8).
7. The `match` over `KaniOutcomeKind` has no wildcard arm (FR-030-AC-7).

## Status

Implemented in `src/kani_terminal.rs`, tests `tc_041_*`. Step 7 is by inspection: the map is one
`match` over `KaniOutcomeKind` with guard arms only on `Inconclusive`, and no wildcard arm.

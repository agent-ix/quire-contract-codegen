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
11. Map `Counterexample` with a non-fault `CallSiteRefusal` and with a `DependencyLockError::Input`,
    each bare and wrapped.
12. Map `Counterexample` with the refusal QSL's `DependencyInput::new` returns for a lock whose
    only defect is one library identity selected twice.

## Expected Results

1. Exactly one value per expressible pair, and none is `Tested` (FR-030-AC-1, FR-030-AC-6).
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
11. The call-site and lock-input refusals are `Inconclusive(ReplayRefused)` carrying their QSL
    catalog code and none is `Declined` (FR-030-AC-12).
12. `Inconclusive(ReplayRefused)` carrying `invalid_package` (FR-030-AC-13).

## Status

Planned. No outcome maps to QSL's terminal value at this revision. Steps 8, 10, 11 and 12 wait on the unmerged
QSL `Inconclusive` terminal value, because they include settlements whose value is
`Inconclusive`; step 9, which includes `InvalidFunction` and `Name` as `Failed`, does not. Step 2
asserts the cause only; the `Declined` code waits on IR-605 and QSL-351, as relayed in FR-030
Status.

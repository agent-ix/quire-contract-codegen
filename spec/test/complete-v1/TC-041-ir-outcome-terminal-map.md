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
refusal causes survive the map, and that the map is one match with no wildcard arm.

## Test Procedure

1. Map an outcome of every `KaniOutcomeKind` and collect the values.
2. Map `Refused`, `InvalidInput` and `IncompleteInput`.
3. Map `TimedOut`, `ResourceExhausted` and `Cancelled`.
4. Map `Proved` (built from one SUCCESS check), `Counterexample` and `Unavailable`.
5. Map `Inconclusive` with cause `kani_vacuous_proof`, and with another cause.
6. Inspect the map's source for a wildcard arm.

## Expected Results

1. Exactly one value per outcome (FR-030-AC-1).
2. `Declined` with three distinct causes (FR-030-AC-2).
3. `Incomplete` with three distinct causes (FR-030-AC-3).
4. `Proved` with one or more checks, `Refuted` and `Unsupported` (FR-030-AC-4).
5. `Proved { success_checks: 0 }` and `Failed` (FR-030-AC-5).
6. No wildcard arm, and no value is `Tested` (FR-030-AC-1, FR-030-AC-6).

## Status

Planned. No outcome maps to QSL's terminal value at this revision.

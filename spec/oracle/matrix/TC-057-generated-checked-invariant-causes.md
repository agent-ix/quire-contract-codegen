---
id: TC-057
title: "Check typed checked-invariant causes from generated oracles"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: verifies
---
# TC-057: Check typed checked-invariant causes from generated oracles

## Description

Planned owner control for FR-042-AC-1's IR-709 generated producer migration.

## Test Procedure

1. Exercise every generated Boolean, equality, function and pre-check placeholder checked-invariant constructor listed in the second cause table of quire-exact FR-369. Inspect the emitted source for each of the nine named paths; do not count an unreachable branch or a fabricated failure as evidence.
2. After IR-595, select an authentic admitted callable with a Boolean result
   from the FR-485 facade. Register a reachable generated body with that
   selection and use a test mutation of the emitted body to return
   `Completed(Value::Integer(...))`. Assert that the body compares against
   that selection's `signature().result()` via the admitted QSV environment
   and reports `GeneratedUnexpectedOutcome`. Run the unmutated Boolean body
   and separate `Undefined`, `Refused` and `Incomplete` controls; assert that
   only the wrong completed value changes cause and that admitted charges are
   unchanged. Do not synthesize an outcome at the adapter boundary.
3. Once QSL-1105 exposes the selected owner's record/tuple members, use a
   checker-admitted source with real nominal declarations to exercise the
   emitted function's QSV environment construction. Record the declaration
   keys, forms, ordered members, presence and types read from that checked
   owner, and verify that the emitted constructor consumes those exact
   declarations. Exercise a genuine construction rejection through the
   emitted path and assert `GeneratedEnvironmentRejected`, the original typed
   diagnostic and zero body operation/charge. A successful construction and
   separate limit and cancellation controls shall not produce this cause.
   No invented declaration, empty default environment or forced constructor
   error counts. If no checker-admitted source reaches a construction
   rejection, this step remains unverified; report the owner-contract gap
   instead of granting acceptance credit.
4. Compare each typed cause and nested `IllTypedCause` against the input result; check the generated charge and failure boundary.
5. Confirm no former unit constructor or invented default remains. The QSL replay consumer is verified at its owner.

## Expected Results

All nine genuine CG producer paths retain their table causes and original nested payloads without changing charge or failure behavior; a reachable wrong-result body and a real owner-declaration construction rejection are shown, and no fabricated failure is counted as evidence. The environment-rejection leg remains PLANNED and blocked by QSL-1105 until its owner contract and a reachable source are verified.

## Status

PLANNED under IR-709.

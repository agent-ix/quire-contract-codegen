---
id: TC-057
title: "Check typed checked-invariant causes from generated oracles"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: verifies
---
# TC-057: Check typed checked-invariant causes from generated oracles

## Description

Planned owner control for FR-042-AC-1 through AC-3 and FR-021-AC-38 through
AC-40 in IR-709's generated producer migration.

## Test Procedure

1. Exercise every generated Boolean, equality, function and pre-check placeholder checked-invariant constructor listed in the second cause table of quire-exact FR-369. Inspect the emitted source for each of the nine named paths; do not count an unreachable branch or a fabricated failure as evidence.
2. After IR-595, select an authentic admitted callable with a Boolean result
   from the FR-485 facade. Register a reachable generated body with that
   selection. Mutate only the test's copy of the emitted body source to return
   `Completed(Value::Integer(...))`, recompile it, and run the normal registered
   `Body`. Keep the shipped result-check code byte-identical in the test
   copy, and verify that it is present in the unmutated production source.
   This is a reachable wrong value supplied to the shipped check, not an
   injected refusal or a production test hook. Assert that the body compares against
   that selection's `signature().result()` via the admitted QSV environment
   and reports `GeneratedUnexpectedOutcome` through `Ok(Transition::Done)`
   with the result occurrence as `Evaluation::location`, not through
   `Err(InternalFault)`. Run the unmutated Boolean body
   and separate `Undefined`, `Refused` and `Incomplete` controls; assert that
   only the wrong completed value changes cause and that admitted charges are
   unchanged. Inspect the generated `admits` result match and the public QSV
   API to verify that an `Err(EnvironmentFailure)` cannot enter the
   `Ok(false)` cause arm. After QSL-1107 supplies a public reservation-control
   seam for this exact body-side call, run an allocation/capacity stop through
   the unchanged `admits` call and assert it is neither
   `GeneratedUnexpectedOutcome` nor `Completed`, with the owner-defined typed
   leaf result. This executable storage control remains PLANNED; the current
   public QSV API cannot inject its private `GlobalReservations` policy.
   Nominal result values
   also wait for QSL-1105; this Boolean control does not claim nominal coverage.
3. Once QSL-1105 exposes the selected owner's record/tuple members, use a
   checker-admitted source with real nominal declarations to exercise the
   emitted function's QSV environment construction. Record the declaration
   keys, forms, ordered members, presence and types read from that checked
   owner, and verify that the unmutated emitted constructor consumes each
   exact declaration once and succeeds. Mutate only the test's copy of the
   emitted pre-body assembly source to submit one unchanged real owner
   declaration a second time to the same shipped QSV constructor. Recompile
   and run the registered body. Assert that the constructor's genuine
   duplicate-key result is exactly
   `EnvironmentFailure::Refused(InvalidDeclaration)` and maps to
   `GeneratedEnvironmentRejected`, with the original typed diagnostic,
   declaration occurrence location and zero body operation/charge. A
   successful construction shall not produce this cause. Exercise the public
   `Limit(WorkUnits)` control with the cancel handle both tripped and
   untripped. Inspect the emitted match and public QSV API to confirm that
   `Allocation` and `Capacity` are separate arms and cannot map to a
   checked-invariant refusal. After QSL-1107 supplies a public
   reservation-control seam for this exact constructor, execute both storage
   failures through the unchanged constructor and assert the owner-defined
   typed leaf result. Those executable storage controls remain PLANNED;
   current public QSV constructors use private `GlobalReservations`.
   No invented declaration, empty default environment, injected constructor
   error or production test hook counts. The healthy source shall retain the
   same checked owner declaration set and succeed.
4. Compare each typed cause and nested `IllTypedCause` against the input result; check the generated charge and failure boundary.
5. Confirm no former unit constructor or invented default remains. The QSL replay consumer is verified at its owner.

## Expected Results

All nine CG producer guards retain their table causes and original nested payloads without changing charge or failure behavior; test-only emitted-source mutations reach the shipped wrong-result and owner-declaration construction guards without injecting a failure. The environment-rejection leg remains PLANNED and blocked by QSL-1105 and QSL-1107 until its owner contracts and the healthy/mutant controls are verified.

## Status

PLANNED under IR-709.

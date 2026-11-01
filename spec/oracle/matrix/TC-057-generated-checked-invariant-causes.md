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

Owner control for FR-042-AC-1 through AC-3 and FR-021-AC-38 through
AC-40 in IR-709's generated producer migration.

## Test Procedure

1. Confirm the retired placeholder is absent and unexpected body resumes retain their fault channel. Exercise every generated Boolean, equality, function checked-invariant constructor listed in the second cause table of quire-exact FR-369. Inspect the emitted source for each of the eight named paths; do not count an unreachable branch or a fabricated failure as evidence.
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
   unchanged.
   Inspect the generated `admits` result match and typed leaf mapping: an
   `Err(EnvironmentFailure)` must never enter the `Ok(false)` cause arm.
   Execute both Boolean and Integer primitive result bodies through the public
   reservation-boundary entry in the test-only emitted copy. Deny every actual
   request, observe zero callbacks, and assert the original completed values:
   these primitive walks allocate nothing. This is **inspection with no
   Allocation/Capacity execution credit**, not a storage-failure test.
   The first shipped body family whose real checker-issued result type and
   completed value make membership request storage activates
   [IR-1235](https://linear.app/agent-ix/issue/IR-1235). In that family's delivery,
   observe a real request and separately deny it as Allocation and Capacity
   through the public QSV seam, retaining the complete typed leaf failure,
   charge prefix and location through the unchanged emitted guard. No fake
   request/failure, substituted result type, production hook or unrelated new
   body family is permitted. Merely allocating the result value does not count
   if its membership walk still reserves nothing.

3. Use a
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
   checked-invariant refusal. Execute both storage failures through the actual constructor reservation
   boundary and assert the owner-defined typed leaf result. Use the public QSV reservation-boundary entry in the test-only emitted
   copy. Preserve the constructor implementation and cause guards; only
   the actual reservation policy is controlled.
   No invented declaration, empty default environment, injected constructor
   error or production test hook counts. The healthy source shall retain the
   same checked owner declaration set and succeed.
4. Compare each typed cause and nested `IllTypedCause` against the input result; check the generated charge and failure boundary.
5. Confirm no former unit constructor or invented default remains. The QSL replay consumer is verified at its owner.

## Expected Results

All eight live CG producer guards retain their table causes and original nested
payloads without changing charge or failure behavior. The wrong-result and
authentic duplicate-declaration controls reach the unchanged shipped guards.
Required and optional owner field presence survives native assembly. Genuine
constructor work/cancel and Allocation/Capacity stops retain typed leaf failures.
Primitive result membership has inspection evidence and explicitly no storage
failure execution credit; the first genuinely allocating membership result
family activates the retained IR-1235 obligation above.

## Status

Native controls implement the current-family obligations under IR-709.
Result-membership storage execution credit is conditional as stated above,
not claimed for the allocation-free Boolean/Integer path.

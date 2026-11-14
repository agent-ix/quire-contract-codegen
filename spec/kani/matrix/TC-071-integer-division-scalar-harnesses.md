---
id: TC-071
title: "Verify integer division and modulo scalar harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-057
    type: verifies
---
# TC-071: Verify integer division and modulo scalar harnesses

## Description

Verify [FR-057](../functional/FR-057-integer-division-scalar-harnesses.md)
through QSL-emitted, IR-admitted packages, public CG generation on both integer
routes, host differentials against quire-exact, installed proof execution and
scalar replay. This case is planned. Run generated proofs only with the
authorized runner and budget (one Kani job at a time, under the caller's memory
cap). This case records measured verdicts and does not predict them.

## Test Procedure

1. Compile QSL sources for `x div y` under each of the truncating, floor and
   Euclidean `integer_division` selections, and `x mod y`, over
   `x: Int[-10, 10]` and `y: Int[-3, 3]`. Admit them through Contract IR.
   Also compile `x rem y` under one law. Generate through `generate_routed` and
   `generate_routed_native`. Assert each `div` and `mod` item settles
   `Supported`, or with one named FR-057 refusal, and never
   `OperationNotRendered`. Assert each `div` harness's profile equals the
   claim's `IntegerDivision { profile }`. Assert the `rem` item stays
   `Underived` with no harness. Inspect the catalog's scalar Kani projection
   (AC-1).
2. Inspect each generated harness: one `i64` draw per operand over its lowered
   range, no assumption on the divisor, the dividend or the outcome, and the
   four-way outcome mapping. Evaluate the rendered assertion on the host over
   the real oracle at `y = 0`, at an in-range member and at an out-of-range
   member, then with the mutated mappings (any `Refused`, `Incomplete`,
   `Completed` at zero). Each mutated mapping must fail (AC-2).
3. Inspect each `div` expected side for the relational window check, the
   division-free admission from `r(L)` and `r(U)`, the quotient-then-divisor
   product order, and the absence of every forbidden token and kernel or
   oracle path. Inspect the `mod` expected side for `rem_euclid` as its only
   division-family operation (AC-3).
4. Compile the rendered expected-side text as host Rust and compare it with
   quire-exact `divide` and `modulo` over `Integer`, per profile, over the
   FR-057-AC-4 grid and result ranges. Assert outcome kind and value agree,
   including the IR-601 vectors (`-7 div 2`, `-7 div -2`, `-7 mod 2`,
   `-7 mod -2`). Swap two window rows and assert the differential fails (AC-4).
5. At `y = 0`, assert `Undefined(DivisionByZero)` from the oracle, from
   quire-exact and from the harness assertion, per profile and for `mod`. Build
   `x div y` with an explicit `Int[i64::MIN, i64::MAX]` result range and assert
   that `i64::MIN div -1` is `Refused(DivisionOutOfDomain { member: Quotient })`
   from both. Build it with the inferred range over `x: Int[i64::MIN, 0]` and
   assert `BoundNotI64` with the two canonical endpoints and no harness. Assert
   `i64::MIN mod -1` completes `0` (AC-5).
6. On the native route, while quire-exact has no native division plan, assert
   the typed native-plan refusal naming node and operation, no harness, no
   `NativeIntegerCapacity`, and a byte-identical supported add sibling (AC-6).
7. For each shape and width FR-057 lists, run the healthy harness through the
   approved bounded path with full ordered options and record the measurement
   fields. For each shape that finished, assert `Verified` with all checks and
   the one final cover satisfied (AC-7). Record each shape that did not finish
   as a `not_tractable` row of the shipped scalar table. Assert the decider
   refuses at and above its width with no Kani launch, runs below it, and
   refuses no measured-verified shape (AC-9).
8. Run each FR-057-AC-8 mutant over a fixture with a negative dividend, both
   divisor signs and zero: the six oracle profile swaps, the wrong-profile
   expected window, `mod` as truncating and as floor remainder, the oracle
   completing at zero, and the expected side without its zero arm. Assert
   `Falsified` with in-domain playback. Replay the decoded operands through the
   routed scalar replay and through quire-exact over `Integer`, and assert the
   kernel value differs from the mutant's (AC-8).
9. Check separate controls for an unsatisfied cover, a failed unwind, a
   timeout, memory exhaustion and a missing verdict against a division
   harness. None may be `Verified` (AC-10).
10. Generate add, subtract, multiply, negate, the integer comparisons,
    implication, enum and float comparison claims before and after the change
    and assert byte-identical output. Re-run FR-015-AC-115's reference-copy
    check and assert the `rem` record is unchanged (AC-11).
11. Inspect the run record for node and source association, operation,
    profile, route, ranges, full argv with unwind options, target, limits,
    checks, cover, playback, exit and terminal outcome (AC-12). Tool versions
    are not required (QSL-406, EA-49).

## Expected Results

Every measured-tractable shape verifies against an expected side that shares no
algorithm with the kernel. Each mutant fails with an in-domain, replayable
disagreement. A zero divisor is `Undefined(DivisionByZero)` and `MIN / -1` is a
typed refusal or a `BoundNotI64` lowering refusal, consistent with the oracle and
the runtime. Unmeasured, not-finished, refused and unrun shapes receive no proof
credit.

## Status

PLANNED (IR-854): no step has run.

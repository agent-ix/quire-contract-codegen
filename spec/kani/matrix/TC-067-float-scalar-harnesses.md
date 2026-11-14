---
id: TC-067
title: "Verify bounded IEEE float scalar operation harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: verifies
---
# TC-067: Verify bounded IEEE float scalar operation harnesses

## Description

Verify [FR-053](../functional/FR-053-float-scalar-harnesses.md) through admitted float
packages (over the quire-exact float carrier, IR-1446, for the comparisons whose quire-exact harnesses
are Verified; the arithmetic identities are refused as not tractable, and the Contract IR float
`requires_bound` ruling, IR-69, would make the unpinned-operand refusal upstream),
public CG generation, installed proof execution and owner replay. FR-014 oracle agreement does not establish Kani acceptance. Execute generated proofs only
with the authorized runner and budget. The control points named in FR-053's Behavior
section (`Z+`, `Z-`, `ONE`, `QN1`, `SN1`, `T1` through `T4`, `M1` and the others) are
used by name below.

## Test Procedure

1. Admit packages covering `numeric_equal`, `total_order`, `bit_identical`, and
   `float32` and `float64` `add`, `sub`, `mul` and `div`, each over parameters of one
   width, the arithmetic result typed by a `float_rounding`. Request each harness and
   assert the operation, width, operand forms and, for arithmetic, the result bound are
   the admitted node's, that no result bound is read for a comparison, and that the
   expected side is not a second call of the generated oracle. Request `sqrt` and `fma`
   and assert the FR-014 derivation (IR-1502, delivered) yields the descriptor, and no
   harness while P1 is absent. Request `to_float32`, `to_float64`, `from_exact`, `to_rational`,
   `float32.negate` and `float64.negate` and assert no harness and the existing dispositions. Request a float literal operand
   and assert `unsupported` naming `FloatLiteralOperand`, and a non-parameter operand and
   assert `unsupported` naming `OperandNotSupported`, each with the node (AC-1).
2. Host-enumerate the AC-2 class-boundary set at each width, plus the `float32` values
   2^32, -1, `i64::MIN` and `i64::MAX`, and compare with the carrier encoding for
   soundness, injectivity and completeness. Mutate by assuming non-negative `float64`
   bits, excluding NaN, excluding infinities, excluding subnormals and zero, leaving the
   `float32` draw unconstrained above `u32::MAX`, decoding `float32` as a signed 32-bit
   value, and canonicalizing a NaN on decode; each must fail the correspondence check at
   its named point. Inspect the FR-025 float row: one `i64` binding per operand, and
   Boolean and integer order unchanged (AC-2).
3. Request an arithmetic node whose result has no `float_rounding`, a wrong-form bound,
   two bounds, a bound unequal to the descriptor, and an unreadable `float_rounding`; and a
   comparison and an arithmetic operand whose type reaches no `float_rounding`. Assert
   `RequiresBound`, `MissingBound`, `AmbiguousBound` and `BoundMismatch` at the oracle
   boundary (`RequiresBound` and `MissingBound` are distinct refusals that both settle as the
   `requires-bound` Kani disposition; `AmbiguousBound` and `BoundMismatch` settle as an oracle
   refusal), then the retained unreadable cause and the operand `requires-bound`, each with no
   harness,
   that no result bound is requested for a comparison, and that a `float_rounding` pin does
   not narrow any operand draw. Record that the unpinned-operand case is upstream
   `requires_bound` only after IR-69 (AC-3).
4. Compare the generated oracle with the independent native expression from matching
   entry Meter state and limits at zero, exact and short budgets (including a short `work_units` budget) at each charge point:
   `ieee.operands`, `ieee.exact-intermediate`, `ieee.round` and `ieee.result-retain`. Run a
   finite input to confirm all four points; a NaN, an infinity, an invalid operation and a
   finite divided by a zero to confirm only `ieee.operands` and `ieee.result-retain`; and
   an `exact` refusal to confirm no `ieee.result-retain`. Assert the arity as value
   occurrences for `sqrt` and `fma` (AC-4).
5. Exercise each reachable noncompletion: `Incomplete` at each charge point an input
   reaches, and under `exact` `Refused(IeeeNotExact)` with the would-be flags over `T1`,
   `MAXF + MAXF`, `SUBMIN x HALF` and `SQ1`. Assert a division by zero and an
   invalid operation under `exact` complete with their flag. Assert no `Undefined` or
   comparison `Refused` control exists, that `DistinctIeeeWidths` has no harness control,
   and inspect the harness for no outcome-based assume (AC-5).
6. Request each arithmetic identity beside a supported comparison in one routed request and
   assert the typed `float.<op>.not_kani_tractable` refusal with no harness, the supported
   sibling unchanged from its solo generation, and total accounting. Exercise an unsupported
   operand form beside a supported sibling and assert its named disposition. A float leaf of a
   composite keeps FR-015's `ShadowFamilyNotBuilt`; the float row rewrites only the
   `OperationNotRendered` record of a float-row identity, and no control for the composite leaf
   runs here (AC-6).
7. Run the healthy harness through the approved bounded production path with its complete
   ordered options and an unwind above the carrier's loop bound. Verify all checks and
   exactly one final cover. Separately force an unsatisfied cover, a failed unwind (an
   unwind at or below the loop bound), a timeout, a memory ceiling below the observed peak
   and a missing verdict, and assert none is `Verified` and the memory case is
   `inconclusive` with its reason, per width, operation and rounding mode (AC-7).
8. Run the AC-8 comparison mutants one at a time, each against only the operators and
   points AC-8 pairs with it: (a) through (c) on `numeric_equal` over `Z+`/`Z-`,
   `QN0`/`QN0` and `ONE`/`MONE`; (d) and (e) on `bit_identical` over `Z+`/`Z-`, `QN0`/`QN0` and
   `QN1`/`QN2` (the only point that catches (e)); (f) through (k) and (m) on
   `total_order` over `MONE`/`MTWO`, `MONE`/`ONE`, `Z+`/`Z-`, `ONE`/`ONE`,
   `INF+`/`QN0`, `NQN0`/`INF-`, `QN0`/`SN1` and `NSN1`/`NQN0`; (l) the swap on
   `total_order` over `ONE`/`TWO` only. Assert no swap claim for `numeric_equal` or
   `bit_identical`. Each must fail with actual in-domain playback (AC-8).
9. Run the AC-9 arithmetic mutants one at a time over the stated points: the operation
   substitution over `THREE`/`TWO`; the swaps (the `add` and `mul` swap only over
   `QN1`/`QN2`); the rounding-mode substitution over `T1` through `T4` for `add` and `fma`,
   `S1` through `S4` for `sub`, `M1` and `M1n` for `mul`, `D1`, `Q1` and `Q1n` for `div` and
   `SQ1` and `SQ2` for `sqrt`, only for the mode pairs the Behavior list says each separates
   (nearest-even against nearest-away, and toward-zero against toward-negative, are not run for
   `sqrt`, being equivalent there);
   the `exact` mutants; zero sign; division by zero; invalid operations; NaN propagation;
   overflow under each directed mode; underflow, flush-to-zero and the subnormal-to-normal
   carry; `fma` as `mul` then `add` at both widths; `sqrt` inexactness; and provenance.
   Each must fail with actual in-domain playback (AC-9).
10. Run the AC-10 charge and outcome mutants (dropped or reordered charge, the rounding
    points charged on a classification-decided input, the wrong width or arity, a charge
    on an `exact` refusal, a stopped outcome reported completed) one at a time (AC-10).
11. Perturb retained witnesses (missing, duplicate, foreign, role-swapped, width-conflicting,
    malformed width or comment, a `float32` value above `u32::MAX` or negative, over
    entry-count or byte bound; for a two-parameter harness set the value-count limit to the
    exact physical entry count, which must be accepted, and to one fewer, which must refuse
    before reading the last entry; apply the same at/under pair to the byte limit) and
    assert typed refusal with no replay. For a valid witness assert one `WitnessValue::Float32`
    or `Float64` per parameter reaches public replay with exact bits, over `Z-`, `SN1`,
    `NQN0` and `0x8000000000000000`, and that a decode through a host float or a checked
    `i64` to `u64` conversion fails. Record an unavailable route as an open dependency
    (AC-11).
12. Inspect the run record for operation and source association, width, rounding mode,
    correspondence, ordered physical bindings, argv, unwind bound and loop bounds, every
    option (no tool versions, QSL-406 and EA-49), target, dependency cohort including the carrier, limits, observed peak memory
    per width, operation and rounding mode, checks, cover, playback, exit and terminal
    classification (AC-12).
13. On the host, first assert every comparison control point's expected result against the
    shipped owner. Then run the carrier's native comparisons and the shipped `compare_ieee` over
    the step 2 enumeration and the control points for every comparison and width, at zero, exact
    and short budgets of every counter a comparison charges, and assert equal outcome, meter,
    denied charge point and consumed counters, and that distinct widths are ill-typed before any
    charge (AC-4, AC-13). The result covers the enumerated and boundary inputs, not every operand
    pair. The arithmetic differential (rounding, flags, provenance, the arithmetic mutants and the
    comparison with Rust's operators) has no harness to accompany and is settled by the refusal
    ruling.
14. Inspect the expected side of each comparison harness and assert no statement of it names a
    carrier function, the bit-pattern decode or the owner's path. Mutate the expected side to call
    the native comparison, then the carrier's classification, then read the operand from the
    carrier; each must fail the inspection while the harness still compares against the oracle
    (AC-14).

## Expected Results

Healthy operations verify with a satisfied cover; every mutant and refusal retains its own
classification and receives no proof credit. Until the carrier and the generated-proof path
run, IR-260 remains unaccepted regardless of oracle agreement.

## Status

PARTIAL (IR-260): steps 1 to 5, 7, 8 and 11 to 14 run for the three comparisons, and step 6 for the refusals and the supported sibling, at both widths in
`tests/it/float_scalar_harness.rs`, with installed-Kani verdicts and owner replay of a decoded
counterexample (`docs/qualification/ir260-float-scalar-kani-measurements.md`). Step 3 runs the
operand `requires-bound` and the unreadable-rounding refusal on hand-edited Contract IR. Step 7 runs
the healthy proof and a forced unsatisfied cover; its other forced failures are the shared runner's
(`tests/it/terminal_map.rs`). Arithmetic is dispositioned not tractable (typed refusal, no
harness), so the arithmetic parts of steps 1, 3, 4, 5, 9, 10, 13 and 14 settle as refusals only; steps 9 and
10 have no comparison part and run nothing, and the composite-leaf control of step 6 is not run. The status of each criterion is in the FR-053 Status.

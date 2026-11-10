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
packages (conditional on the quire-exact float carrier, IR-1446; the Contract IR float
`requires_bound` ruling, IR-69, for the upstream half of the unpinned-operand refusal;
and, for `sqrt` and `fma`, the FR-014 derivation of IR-1502; none is delivered),
public CG generation, installed proof execution and owner replay. This case is planned;
FR-014 oracle agreement does not establish Kani acceptance. Execute generated proofs only
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
   and assert `OperationNotDerivable` and no harness while the FR-014 extension
   (IR-1502) is absent. Request `to_float32`, `to_float64`, `from_exact`, `to_rational`,
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
6. Remove the carrier and assert `OperationNotRendered` (`NoRenderer`, no prerequisite
   name). Present `sqrt` and `fma` without the FR-014 extension and assert
   `OperationNotDerivable`. Exercise an unsupported operand form, an absent native
   expression and a float composite leaf beside a supported sibling and assert total
   accounting and the leaf's `ShadowFamilyNotBuilt` (AC-6).
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
    correspondence, ordered physical bindings, argv, unwind bound and loop bounds, tool
    versions, target, dependency cohort including the carrier, limits, observed peak memory
    per width, operation and rounding mode, checks, cover, playback, exit and terminal
    classification (AC-12).
13. On the host, first assert every control point's expected result against the shipped
    owner. Then run the carrier's `cfg(kani)` replacement of `evaluate_ieee` and
    `compare_ieee` and the shipped owner functions over the step 2 enumeration and the AC-13
    boundary family for every supported operation, width and rounding mode, at zero, exact
    and short budgets, and assert equal outcome, bits, flags, provenance, ordered charges,
    denied next charge and counters. Confirm the carrier makes its replacement callable from
    the host build (P1). For `nearest-even`, compare the non-NaN results with Rust's `f32`
    and `f64` operators, `sqrt` and `mul_add`. Mutate the replacement's ordering, charge
    sequence, flags, provenance, rounding step, sticky shift and operand classification (each
    must fail this differential and the proof against the expected side), and mutate the
    bit-pattern decode shared by both sides (it fails only this differential) (AC-13). This step cannot run until
    IR-1446 supplies the host-callable replacement.
    The result covers the enumerated and boundary inputs, not every operand pair.

14. Inspect the call graph of each supported operation's expected side and assert the
    bit-pattern decode is the only function it shares with the carrier's `cfg(kani)`
    replacement. Mutate the expected side to call the replacement's rounding step and then its
    operand classification; each must fail the inspection while still passing the "not a second
    call of the oracle" check of step 1 (AC-14).

## Expected Results

Healthy operations verify with a satisfied cover; every mutant and refusal retains its own
classification and receives no proof credit. Until the carrier and the generated-proof path
run, IR-260 remains unaccepted regardless of oracle agreement.

## Status

PLANNED: all fourteen FR-053 criteria are unimplemented and untagged. No missing carrier
capability (IR-1446), Contract IR ruling (IR-69), FR-014 `sqrt` and `fma` derivation
(IR-1502), replay route or generated-proof result is waived. No width, operation or rounding
mode is known to verify within a request's ceilings.

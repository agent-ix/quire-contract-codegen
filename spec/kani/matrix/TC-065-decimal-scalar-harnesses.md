---
id: TC-065
title: "Verify bounded decimal scalar operation harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: verifies
---
# TC-065: Verify bounded decimal scalar operation harnesses

## Description

Verify [FR-051](../functional/FR-051-decimal-scalar-harnesses.md) through admitted
decimal packages (conditional on the quire-exact decimal carrier, IR-1444, which is
not delivered), public CG generation, installed proof execution and owner replay.
This case is planned; FR-014 oracle agreement does not establish Kani acceptance.
Execute generated proofs only with the authorized runner and budget.

## Test Procedure

1. Admit packages covering decimal `add`, `sub`, `mul`, `div`, `negate`,
   `convert_rounding` and `lt`, `le`, `gt`, `ge`, each with `decimal_range` operand
   bounds, and a result bound for the six arithmetic operations only (orderings have
   none). Request each harness and assert the operation, operand forms (and, for the
   arithmetic operations, the result bound) are the admitted node's, and that the
   expected side is not a second call of the generated oracle. Request `decimal.eq` and `decimal.ne` and
   assert no harness and FR-014's `OperationNotDerivable` (AC-1).
2. Independently enumerate each operand's admitted (coefficient, scale) set at scales
   `0..=scale_max` with exact host arithmetic and `DecimalType::contains`, and
   include the owner's own result form (`(5000, 2)` for 50 in `Decimal[0,100;0,2]`,
   a raw coefficient outside the declared interval). Compare with the carrier
   encoding for soundness, injectivity and completeness, including numerically equal
   pairs with different scales. Mutate by clipping an endpoint, rejecting a `contains`-admitted raw
   coefficient outside the interval, admitting a pair `contains` rejects, collapsing equal-valued pairs and fixing the scale; each must
   fail the correspondence check. Inspect the FR-025 Decimal row: two `i64`
   bindings, coefficient then scale, in ascending logical order, with Boolean and
   integer order unchanged (AC-2).
3. Compare the generated oracle with the independent native expression from
   matching entry Meter state and limits: outcome, retained representation, loss
   record, ordered charges, denied next charge and counters at zero, exact and short
   budgets at `decimal.operands`, `decimal.scale-expansion`, `decimal.arithmetic`,
   `decimal.rounding`, `decimal.result-retain`, and for comparisons
   `ordering.operands`, `ordering.arithmetic` and `ordering.result-retain`.
   Include `1.0` against `1` to confirm ordering by value (AC-3).
4. Exercise each reachable noncompletion: zero divisor (and the same input at a
   zero budget, which stops `Incomplete` at `decimal.operands`); under `exact`
   rounding, `InexactDecimal` for one third at scale two, a `mul` whose operand
   scales sum above the result scale, and `add`, `sub`, `negate` and
   `convert_rounding` over an operand whose scale exceeds the result scale;
   `DecimalOutOfDomain` for `add`, `sub`, `mul`, `div`, `negate` and
   `convert_rounding`, including one reached after rounding; and each `Incomplete`
   charge point. Assert the agreed typed outcome and charges, that comparisons
   yield only `Completed` or `Incomplete`, and inspect the harness for no
   outcome-based assume (AC-4).
5. Request a missing, unbounded and repeated `decimal_range`, and, for an arithmetic
   operation only, a missing and an ambiguous result bound (an ordering needs none), a lifted coefficient bound `max(|lo|,|hi|) x 10^(scale_max - scale_min)` beyond
   `i64` (for example `Decimal[0,10^18;0,2]`, whose endpoints fit but whose lifted
   bound is 10^20), and a scale beyond the carrier. Assert named `requires-bound` or `unsupported` retaining the
   endpoint and lifted bound, with no harness and no clipping. Remove the carrier and assert
   `OperationNotRendered` (`NoRenderer`, no prerequisite name). Exercise an
   unsupported operand form, an absent native expression and a decimal composite
   leaf beside a supported sibling and assert total accounting (AC-5, AC-6).
6. Run the healthy harness through the approved bounded production path with its
   complete ordered options. Verify all checks and exactly one final cover.
   Separately force an unsatisfied cover, failed unwind, timeout and missing verdict
   and assert none is `Verified` (AC-7).
7. Run the AC-8 mutants one at a time, each over a control domain stating its
   distinguishing point and only against the operators FR-051-AC-8 pairs with it:
   (a) add/sub; (b) swap in `sub`, `div` and the four orderings over numerically distinct operands
   (`div`: nonzero, `a` not `-b`), with no swap claim for `add` or `mul`; (c) `lt`/`le`, `gt`/`ge`
   at equal values; (d) raw-coefficient-only ordering over `1.0` (10, scale 1) and `2` (2, scale 0); (e) rounding
   mode over the 0.125 tie; (f) normalized-value result or wrong scale over
   `0.5 + 0.5`; (g) dropped loss record; (h) dropped or reordered charge; (i)
   `InexactDecimal` reported rounded; (j) zero divisor reported a value; (k) stopped
   reported completed. Each must fail with actual in-domain playback (AC-8).
8. Perturb retained witnesses (missing, duplicate, foreign, role-swapped, malformed
   width or comment, negative or oversize scale, out of domain, over entry-count or
   byte bound; each decimal parameter has two entries, so for two parameters (four
   entries) set the value-count limit to four, which must be accepted, and to three,
   which must refuse before reading the fourth entry rather than group it; apply the
   same at/under pair to the byte limit) and assert typed refusal with no replay and no normalization.
   For a valid witness assert one `WitnessValue::Decimal` with the unnormalized pair
   per parameter reaches public replay for that declaration. Record an unavailable
   route as an open dependency (AC-9).
9. Inspect the run record for operation and source association, bounds and
   correspondence, ordered physical bindings, argv, unwind bounds, tool versions,
   target, dependency cohort including the carrier, limits, checks, cover, playback,
   exit and terminal classification (AC-10).

## Expected Results

Healthy operations verify with a satisfied cover; every mutant and refusal retains
its own classification and receives no proof credit. Until the carrier and the
generated-proof path run, IR-1440 and IR-259 remain unaccepted regardless of oracle
agreement.

## Status

PLANNED: all ten FR-051 criteria are unimplemented and untagged. No missing carrier
capability, replay route or generated-proof result is waived.

---
id: SR-17940486709564847047584
title: "IR-1440 PR #766 FR-051 failure-domain analysis"
type: SpecReview
analysis: failure-domain
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #766 @ 6c9302b6; IR-1440; spec/kani/functional/FR-051-decimal-scalar-harnesses.md, spec/kani/matrix/TC-065-decimal-scalar-harnesses.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-036-generated-subject-abi.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-065
    type: reviews
---

## Summary

Ticket: IR-1440. Spec-only PR #766 (head 6c9302b6) adding FR-051 and TC-065, the planned Kani row for decimal scalar operations, with edits to FR-025, FR-028, TC-036, TC-030, spec/kani/matrix/tests.md and spec/spec.md. Reviewed against origin/main 66e7eac3 (includes #764 FR-049 and #765 FR-050), quire-exact decimal.rs/numeric.rs/outcome.rs/accounting.rs, CG src/oracle/scalar/derive.rs and src/kani, qsl-replay WitnessValue, and Contract IR FR-038.

## Verdict

CHANGES REQUIRED. The zero-divisor/zero-budget ordering, InexactDecimal-before-rounding-charge and DecimalOutOfDomain-before-result-retain placements are correct against decimal.rs evaluate(). Two gaps: the reachable-noncompletion enumeration is incomplete, and the drawn representation set excludes the owner's own retained form.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-4 says 'each reachable noncompletion' but enumerates InexactDecimal only 'for an inexact quotient or product' and DecimalOutOfDomain only 'for an add, mul or negate result'. In decimal.rs evaluate(), any operation whose intermediate is not on the target scale grid takes the RoundingStep path: convert_rounding under `exact` (the canonical InexactDecimal case) and add/sub/negate whose operand scale exceeds the result smax (operand bounds need not lie inside the result bound, FR-014) also refuse InexactDecimal; sub, div and convert_rounding results can equally be DecimalOutOfDomain. TC-065 step 4 inherits the partial list. Enumerate per operator or state the conditions instead of examples. | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:154 |
| FND-002 | medium | The draw constrains the raw retained pair to the decimal_range coefficient interval [lo, hi] and scale interval [smin, smax] plus DecimalType::contains. But contains bounds the coefficient lifted to s* = max(s, smin), and the owner's own retain() yields (c x 10^(T-s), T) at T = smax. When smin < smax these differ: in Decimal[0,100; 0,2] the member value 50 is retained by the owner as (5000, 2), which the envelope excludes, while (50, 0) is drawn. So the 'every pair so admitted' domain omits representations the owner itself produces for values of the operand's type, and the proof says nothing about them. State the chosen representation set and why it is the parameter's admitted set, or derive it from contains plus the scale interval with a separate i64 capacity check. | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:152 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-051-AC-2 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:152 | Each operand is drawn as a (coefficient, scale) pair inside its inclusive `decimal_range` coefficient and scale intervals and satisfies the owner `DecimalType::contains`. Every pair so admitted is reachable exactly once, including numerically equal pairs with different scales, and an independent host enumeration of the admitted set matches the encoding; clipping an endpoint, admitting a pair that `contains` rejects, collapsing equal-valued pairs to one representation or drawing only the target scale fails the correspondence check (QSpec FR-196-AC-5). |
| FR-051-AC-4 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:154 | Each reachable noncompletion is asserted as the agreed typed outcome and never assumed away: `Undefined(DivisionByZero)` for a zero divisor (and `Incomplete` at `decimal.operands` instead at a zero budget, since the zero check follows that charge), `Refused(InexactDecimal)` for an inexact quotient or product under `exact` rounding, `Refused(DecimalOutOfDomain)` for an add, mul or negate result outside the result domain, and `Incomplete` at each charge point named in the Behavior list. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no a... |
| FR-051-behavior-representation | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:79 | When the operation is representable, the harness SHALL draw each operand as a retained representation, a coefficient and a scale, from the node's `decimal_range` through the carrier. The owner's operations charge and place by the retained representation (`Decimal::representation()`), and `DecimalResult` identity is the retained representation plus the loss record, so two numerically equal operands with different scales (`1.0` and `1`) are distinct inputs. The representation constraint is the owner's value-only membership `DecimalType::contains` over the drawn pair, plus the inclusive coeffi... |
| FR-051-behavior-outcomes | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:98 | The typed outcomes the owner produces are these. `Completed` (with a loss record when a rounding step ran). `Undefined(DivisionByZero)` for a divisor whose normalized coefficient is zero, produced after the `decimal.operands` charge. `Refused(InexactDecimal)` when the result is inexact at the target scale under `exact` rounding, before any `decimal.rounding` charge or loss record. `Refused(DecimalOutOfDomain)` when the placed result is outside the result `DecimalType`. `Incomplete` at a charge point: `decimal.operands`, `decimal.scale-expansion`, `decimal.arithmetic`, `decimal.rounding` (on... |
| FR-025-decimal-row | examined | spec/kani/functional/FR-025-generated-subject-abi.md:74 | / Bounded Decimal (planned [FR-051](./FR-051-decimal-scalar-harnesses.md) scalar operations only) / two `i64` calls: coefficient then scale, inside the original `decimal_range` intervals and constrained by owner `DecimalType::contains`; the pair is the retained representation / one owner `Decimal::new(Integer::from(c), s)` value for the original parameter / |
| TC-065-step-4 | examined | spec/kani/matrix/TC-065-decimal-scalar-harnesses.md:42 | 4. Exercise each reachable noncompletion: zero divisor (and the same input at a    zero budget, which stops `Incomplete` at `decimal.operands`), `exact` rounding    over one third at scale two (`InexactDecimal`), an add, mul and negate result    outside the result domain (`DecimalOutOfDomain`), and each `Incomplete` charge    point. Assert the agreed typed outcome and charges, and inspect the harness for    no outcome-based assume (AC-4). |
| FR-051-AC-9 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:159 | Counterexample decoding validates two physical I64 entries per decimal parameter (width, little-endian bytes, comment, order, schema role, byte and value-count limits), then the scale range, the original domain and `DecimalType::contains`, before grouping. Grouping yields one `WitnessValue::Decimal { coefficient, scale }` per parameter through the public replay route for that exact declaration, retaining the unnormalized pair. Missing, duplicate, foreign, role-swapped, domain-conflicting and malformed schemas or entries, a negative or oversize scale and an out-of-domain pair are refused wit... |

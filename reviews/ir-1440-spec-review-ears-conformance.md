---
id: SR-17940486709564847047582
title: "IR-1440 PR #766 FR-051 EARS conformance analysis"
type: SpecReview
analysis: ears-conformance
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

PASS. The FR-051 statement and each Behavior and ABI paragraph use the event pattern ('When ..., CG/the harness SHALL ...') with a named subject; quire's grammar checks emit no ears: or ac: warning for FR-051, TC-065 or the edited FR-025/FR-028 lines (make spec warnings are pre-existing, in FR-015, FR-017, FR-014 and FR-021).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-051 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:31 | When an admitted request selects a reader-admitted `decimal` scalar operation (`quire.op.decimal.add`, `.sub`, `.mul`, `.div`, `.negate`, `quire.op.numeric.convert_rounding` into a decimal, or an ordering comparison `lt`, `le`, `gt`, `ge` over decimals) and the planned prerequisite below is available, CG SHALL generate a bounded Kani harness that proves the generated decimal oracle of [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with an independent native expression of the same operation over every admitted symbolic decimal operand. This is the decimal row of the ... |
| FR-051-behavior-representation | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:79 | When the operation is representable, the harness SHALL draw each operand as a retained representation, a coefficient and a scale, from the node's `decimal_range` through the carrier. The owner's operations charge and place by the retained representation (`Decimal::representation()`), and `DecimalResult` identity is the retained representation plus the loss record, so two numerically equal operands with different scales (`1.0` and `1`) are distinct inputs. The representation constraint is the owner's value-only membership `DecimalType::contains` over the drawn pair, plus the inclusive coeffi... |
| FR-051-behavior-outcomes | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:98 | The typed outcomes the owner produces are these. `Completed` (with a loss record when a rounding step ran). `Undefined(DivisionByZero)` for a divisor whose normalized coefficient is zero, produced after the `decimal.operands` charge. `Refused(InexactDecimal)` when the result is inexact at the target scale under `exact` rounding, before any `decimal.rounding` charge or loss record. `Refused(DecimalOutOfDomain)` when the placed result is outside the result `DecimalType`. `Incomplete` at a charge point: `decimal.operands`, `decimal.scale-expansion`, `decimal.arithmetic`, `decimal.rounding` (on... |
| FR-051-AC-1 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:151 | Each admitted decimal `add`, `sub`, `mul`, `div`, `negate`, `convert_rounding` and each of `lt`, `le`, `gt`, `ge` node selects a harness naming the operation identity, operand forms and the `decimal_range` result bound from the admitted node; the oracle is FR-014's and the expected side is an independent native expression, not a second call of the oracle. `quire.op.decimal.eq` and `.ne` select no harness and keep FR-014's `OperationNotDerivable`. |
| FR-051-AC-5 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:155 | A missing, unbounded or repeated `decimal_range` for an operand, or a missing or ambiguous result bound, yields source-associated `requires-bound` naming the operand or result with no harness. A coefficient endpoint beyond signed `i64`, or a scale endpoint beyond the carrier or `u32`, yields named `unsupported` retaining the endpoint; generation never clips or substitutes a smaller domain. |
| FR-051-AC-9 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:159 | Counterexample decoding validates two physical I64 entries per decimal parameter (width, little-endian bytes, comment, order, schema role, byte and value-count limits), then the scale range, the original domain and `DecimalType::contains`, before grouping. Grouping yields one `WitnessValue::Decimal { coefficient, scale }` per parameter through the public replay route for that exact declaration, retaining the unnormalized pair. Missing, duplicate, foreign, role-swapped, domain-conflicting and malformed schemas or entries, a negative or oversize scale and an out-of-domain pair are refused wit... |

---
id: SR-17940486709564847047583
title: "IR-1440 PR #766 FR-051 dependency analysis"
type: SpecReview
analysis: dependency
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

CHANGES SUGGESTED. depends_on edges to FR-014, FR-015, FR-016, FR-017, FR-025, FR-028, FR-029 and FR-039 match every requirement FR-051 delegates to; IR-1444 (quire-exact Kani-tractable decimal carrier) exists in Backlog and is correctly stated as the only build prerequisite; `decimal_range` is an admitted bound form today, so no CG-side bound ticket is needed. Two low gaps.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-051 cites FR-046 (AC-4 'inspected, as FR-046-AC-4'), FR-050 (scope exclusion) and QSpec FR-196-AC-5 (AC-2 correspondence) in body text with no `references` relationship, unlike Contract IR FR-038 which has one. Add references edges so the impact graph sees them. | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:5 |
| FND-002 | low | 'an unbounded decimal is `requires_bound` upstream (Contract IR FR-038)' is stated unconditionally, but Contract IR FR-038 raises requires_bound only 'when the profile requires bounds' (FR-038 lines 4954, 4978). Under another profile the unbounded decimal reaches CG, where FR-014 refuses it RequiresBound or MissingBound (both map to the requires-bound disposition in negotiate.rs). State the profile condition, or cite FR-014 as the CG-side guarantee. | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:167 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-051-frontmatter | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:5 | relationships: - target: ix://agent-ix/quire-contract-codegen/StR-001 type: satisfies - target: ix://agent-ix/quire-contract-codegen/FR-014 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-015 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-016 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-017 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-025 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-028 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-029 type: depends_on - target: ix:/... |
| FR-051-dependencies | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:164 | - [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns decimal   oracle semantics and the derivable operation list; its oracle agreement is not   this proof. - K-EXACT-decimal (IR-1444) is prerequisite P1. No Contract IR bound requirement is   outstanding for decimals: `decimal_range` is admitted today, and an unbounded   decimal is `requires_bound` upstream (Contract IR FR-038). - TC-030's Decimal scalar row owns today's `OperationNotRendered` settlement; this   row supersedes it for the listed decimal operations only when delivered. - [FR-015](./FR-015-bounded-kani-obliga... |
| FR-051-AC-4 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:154 | Each reachable noncompletion is asserted as the agreed typed outcome and never assumed away: `Undefined(DivisionByZero)` for a zero divisor (and `Incomplete` at `decimal.operands` instead at a zero budget, since the zero check follows that charge), `Refused(InexactDecimal)` for an inexact quotient or product under `exact` rounding, `Refused(DecimalOutOfDomain)` for an add, mul or negate result outside the result domain, and `Incomplete` at each charge point named in the Behavior list. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no a... |
| FR-051-AC-2 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:152 | Each operand is drawn as a (coefficient, scale) pair inside its inclusive `decimal_range` coefficient and scale intervals and satisfies the owner `DecimalType::contains`. Every pair so admitted is reachable exactly once, including numerically equal pairs with different scales, and an independent host enumeration of the admitted set matches the encoding; clipping an endpoint, admitting a pair that `contains` rejects, collapsing equal-valued pairs to one representation or drawing only the target scale fails the correspondence check (QSpec FR-196-AC-5). |
| FR-051-description-exclusions | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:52 | Decimal equality and inequality (`quire.op.decimal.eq`, `.ne`) are outside this requirement: FR-014 refuses them as `OperationNotDerivable`, so no oracle exists to prove. Quantity carried Decimal conversions are FR-050 and FR-014-AC-62. |

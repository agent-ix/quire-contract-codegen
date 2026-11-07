---
id: SR-2026110283895
title: "Base spec review of IR-1376 bound criteria"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md"
review_set: subset
---
## Summary

Reviewed the changed result-bound description, FR-014-AC-21, TC-024 step 8, and neighboring FR-014-AC-18, AC-22, and AC-25 obligations. The new sole-bound rule is explicit and testable, but its refusal conflicts with the existing plain-reference obligation when both defects occur together.

## Verdict

**FAIL** — the specification assigns two different typed refusals to the same admitted node shape.

## Examined scope

- FR-014-AC-18: derivation and generation have matching typed refusals.
- FR-014-AC-21: a sole operand-owned bound gives `MissingBound`.
- FR-014-AC-22: two or more reachable bounds without exactly one free bound give `AmbiguousBound`.
- FR-014-AC-25: a plain reference beside a bounded operand gives `RequiresBound`.
- TC-024 steps 8 and 11.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-21 and AC-25 require conflicting refusals for `a + p` with one operand-owned bound | spec/oracle/functional/FR-014-exact-scalar-oracles.md:636 |

For `a + p` with scalar `Integer` result, `a` typed by `Int[0, 9]`, and `p` typed by plain `Integer`, the only reachable bound is `a`'s own. AC-21 now requires `MissingBound`, while AC-25 requires `RequiresBound` for a plain reference where another operand owns a bound. TC-024 step 11 and its existing test assert the latter. State which refusal takes precedence when both conditions hold; then keep the code and test aligned with that decision.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-21 now excepts plain references governed by AC-25; AC-25 states `RequiresBound` precedence, and TC-024 steps 8 and 11 distinguish literal and plain-reference operands. |

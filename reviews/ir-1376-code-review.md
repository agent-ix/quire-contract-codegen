---
id: SR-2026110283893
title: "Code review of IR-1376 scalar result bound"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/oracle/scalar/lower.rs, tests/it/exact_scalar_generation.rs, tests/it/exact_function_agreement.rs, tests/exact_function_support/agreement_cases.rs, tests/exact_scalar_support/package.rs"
review_set: subset
---
## Summary

Reviewed the frozen PR #755 Rust diff, its public refusal behavior, admitted V2 fixture, generated native agreement, and existing FR-014-AC-25 oracle. The new result-bound rule is correct for a sole operand-owned bound, but changes the refusal precedence for a different existing case.

## Verdict

**FAIL** — the existing `a + p` scalar-result case now returns `MissingBound` before the required `RequiresBound` check.

## Examined scope

- FR-014-AC-18: derive descriptors and use the same typed refusal as generation.
- FR-014-AC-21: a sole operand-owned bound is not a result bound and yields `MissingBound`.
- FR-014-AC-25: a plain reference beside a bounded operand yields `RequiresBound` in generation and derivation.
- FR-021-AC-35: generated function and independent QSL agreement.
- TC-024 step 8 and the existing AC-25 case.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Sole-bound selection preempts the required plain-operand `RequiresBound` refusal | src/oracle/scalar/lower.rs:1153 |

`TWO_PARAMETER_PLAIN_SCALAR_RESULT` is `a + p`: `a` owns the only reachable `[0, 9]` bound, `p` is a plain `Integer` reference, and the result is scalar typed. `check_parameters_from_view` calls `bounds.equal` before `integer_operand_bounds`, so the new `[bound]` arm returns `MissingBound` at lines 1161-1164. FR-014-AC-25 and the pre-existing `tc_024_ac25_a_plain_typed_reference_operand_beside_bounded_typing_requires_a_bound` demand `RequiresBound` for this exact case in generation and derivation. Preserve that refusal precedence while refusing `a + 1` with `MissingBound`.

## Review boundaries

No tests or gates were run in this review; the owning lane reported its focused tests and will run the final gate. No CI workflow or public Rust signature changed. The changed function is `pub(super)`; the observable public effect is its typed refusal.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The sole operand-owned integer-bound arm now calls `integer_operand_bounds` and returns its `RequiresBound` refusal before `MissingBound`; the AC-25 `a + p` case remains explicit, while the literal case returns `MissingBound`. |

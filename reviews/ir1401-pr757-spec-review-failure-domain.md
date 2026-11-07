---
id: SR-17940486709564847047574
title: "IR-1401 PR757 spec-review-failure-domain"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; IR-1401; src/oracle/scalar/lower.rs; tests/it/scalar_source_forms.rs; tests/it/main.rs; spec/oracle/functional/FR-014-exact-scalar-oracles.md; spec/oracle/matrix/TC-024-exact-scalar-oracles.md"
review_set: subset
---

## Summary

AC-68 preserves strict-reader versus CG failure ownership, typed refusals, supported-sibling isolation and CallerDeclared mismatch behavior. AC-66 preserves carried unit/carrier/rounding identity; AC-67 preserves operation/width/operand order; AC-69 covers denied budgets. No new callback, graph traversal or mutable evaluator is introduced.

Frozen candidate 8512ca2536342d46924c2c63868f808ce8719ab4 against origin/main cd184a37. No dependency, CI, public API or lockfile changes. No applicable AssuranceProfile exists under spec/. Full gates and proof execution were excluded by the lead; existing focused execution logs were inspected, not rerun after the reported cache deletion. No cache or worktree was removed by this reviewer.

## Verdict

**PASS** — No new defect found in this method’s changed scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Frozen candidate 8512ca2536342d46924c2c63868f808ce8719ab4 against origin/main cd184a37. No dependency, CI, public API or lockfile changes. No applicable AssuranceProfile exists under spec/. Full gates and proof execution were excluded by the lead; existing focused execution logs were inspected, not rerun after the reported cache deletion. No cache or worktree was removed by this reviewer.

Reviewed all changed source/tests/spec files. Source uses one shared shape and operand predicate in both lowerer validation points. Existing caller-declared descriptor policy is preserved. No duplicated catalog, arbitrary-family admission or disabled gate was introduced.

Evidence inspected: /tmp/ir1401-final-focused.log (4 pass, 1 fail); /tmp/ir1401-red.log (IEEE FormMismatch and carried reader failure); /tmp/ir1401-clippy.log and /tmp/ir1401-fmt.log (pass); independently computed /tmp/ir1401-review-matrix.json and /tmp/ir1401-review-spec.log. No Kani or backend proof was executed.

## Examined units

- FR-014-AC-66 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: For a strict-reader-admitted `quire.op.quantity.convert`, CG accepts a referenced `scalar_type`/`carried_quantity` source operand permitted by the owner operation contract, including the genuine QSL-emitted TC-187 U01 form, and derives and generates an `ir_confirmed` oracle. Source carrier, unit and rounding and the admitted exact or carried target identity and domain remain intact; execution uses the existing owner conversion contract without replacing a carried source with an unbounded or exact carrier.
- FR-014-AC-67 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: For strict-reader-admitted `quire.op.ieee.numeric_equal`, `quire.op.ieee.total_order` and `quire.op.ieee.bit_identical` nodes, CG accepts the genuine QSL-emitted `expression`/`call` semantic form with the catalogued `call` body operator, including numeric equality emitted from `a = b` and each `quire::value::ieee::*` intrinsic. Existing reader-admitted binary-semantic-form fixtures with that same catalogued call operator remain supported. Derivation and generation retain operand order, float width, Boolean result, operation identity, laws and mode, and dispatch the selected owner IEEE operatio
- FR-014-AC-68 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: Acceptance of these carried operands and IEEE call forms preserves the existing strict-reader and CG boundaries. Unadmitted operation/body pairings and reader-invalid arity, types, identity, laws or mode remain reader refusals. Admitted shapes, arity, operand/result types, unitless quantity literals and unsupported semantic forms that CG previously refused retain typed refusal, no oracle and supported-sibling isolation. A same-lowering-shape descriptor identity, law or mode mismatch retains the existing FR-014-AC-12 and FR-038-AC-3 behavior: where generation was permitted, its operation remain
- FR-014-AC-69 — role: examined — spec/oracle/functional/FR-014-exact-scalar-oracles.md — excerpt: Qualification of AC-66 and AC-67 emits genuine QSL source through the public facade, passes its unchanged emitted bytes through the strict Contract IR reader, derives descriptors and generates, compiles and executes the resulting native scalar oracles. Tests assert completed conversion values and units and IEEE true and false values, including signed-zero and NaN cases that distinguish numeric equality, total order and bit identity according to the owner contract. Generated and direct owner executions agree in complete, refused or incomplete outcome, ordered admitted charges and consumed count

TC-024 added genuine-source qualification procedure examined in full; prior AC-12 / FR-038-AC-3 and AC-62–65 were context only. No integrity/dependency/object lens applies: no new object, edge or structural artifact schema was added. Criterion-strength skipped because not selected; no Jev client invoked.

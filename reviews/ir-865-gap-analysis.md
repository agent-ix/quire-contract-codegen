---
id: SR-12336
title: "IR-865 CG #427 gap analysis: trace attribute migration"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@472397829f3d453fcd426a1160a7179574027b7c; six changed Rust test files and computed coverage/matrix"
review_set: subset
---

## Summary

Ticket: IR-865. PR: quire-contract-codegen#427. Plan completion: not assessed. This PR changes trace syntax on existing tests and no production code or spec. The planless gap audit compared computed coverage and matrix at base `836b298` and head `4723978`.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| `tests/checked_package_support/rekey.rs` | examined | Five migrated test binders. |
| `tests/composite_equality_support/package.rs` | examined | Two migrated test binders. |
| `tests/it/composite_equality_agreement.rs` | examined | One migrated test binder. |
| `tests/it/composite_original_eq.rs` | examined | Sixteen migrated test binders. |
| `tests/it/no_generation_panics.rs` | examined | Seven migrated test binders. |
| `tests/it/terminal_map.rs` | examined | Thirty-one migrated test binders. |
| Computed `quire coverage --scope . --strict` | examined | Both trees: `Coverage: 499/855 rows backed (58%)`; `221 unbacked row(s) and 0 contradicted status(es)`. |
| Computed `quire matrix --scope .` | examined | Both trees: 1,013 output lines, with the same rows and binders after binder line-number normalization. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS** for the PR diff. Paired two-thread base/head integration runs each passed 594 tests with 0 failures and 34 ignored. Default-thread comparison is pending as an addendum to classify the author's seven-failure report. The trace migration itself changes no matrix or coverage result. Existing unbacked rows are unchanged.

## Coverage

Plan completion: not assessed. No production function was added or changed; reverse code-to-spec scope is unchanged. No test assertion or body changed, so the existing evidence remains the same. The optional semantic review was not requested; exact tag migration and test bodies were inspected.

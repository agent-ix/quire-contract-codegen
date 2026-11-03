---
id: SR-202611020195
title: "IR-1289 result-binder refusal gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #719 at 85c153912ae67b6a3ac7464fd9d08efeffd73138; IR-1289; FR-015-AC-39, tagged tests and scalar V2 lowerer"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Checked the FR-015-AC-39 partial behavior against the computed Test Matrix, both new trace-tagged integration tests, and the changed scalar V2 lowerer. No applicable AssuranceProfile exists under `spec/`.

## Verdict

**PASS for the scoped fail-closed slice.** FR-015-AC-39 remains PARTIAL; positive produced-result and typed subject invocation are future work.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed. The computed matrix marks FR-015-AC-39 tagged by `qsl_result_binder_is_not_drawn_as_a_postcondition_parameter` and `qsl_unused_result_binder_refuses_postcondition_harness` in `tests/it/kani_obligations_state_frame.rs`. Both carry TC-025. The tests use the real QSL package compilation and the public CG request. The new typed reason has FR-015-AC-39 as its owning criterion, and no new stub or unowned behavior was found in the changed scope. This diff-scoped verdict does not claim repository-wide criterion completion.

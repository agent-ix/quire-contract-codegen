---
id: SR-202611020192
title: "IR-948 AC-39 multi-parameter V2 precondition gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #718 at 5d2db23b77d0d766547bc85254335725ed1c5f00; FR-015-AC-39, tagged tests and V2 precondition lowerer"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Checked the FR-015-AC-39 criterion against the computed Test Matrix, the two new tagged integration tests, the changed precondition count guard, and the existing renderer. No applicable AssuranceProfile exists under `spec/`.

## Verdict

**PASS for the scoped partial slice.** The criterion remains PARTIAL; state-field draws and postcondition produced-value/subject handling are outside this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed. The computed matrix marks FR-015-AC-39 tagged by `qsl_two_parameter_precondition_draws_each_declared_domain` and `qsl_two_parameter_precondition_refuses_unrenderable_domains` in `tests/it/kani_obligations_state_frame.rs`. Both carry TC-025 as well. The production change has FR-015-AC-39 as its owner. No new stub or unowned behavior was found in the changed scope. This is a diff-scoped review, not a claim that every repository criterion is complete.


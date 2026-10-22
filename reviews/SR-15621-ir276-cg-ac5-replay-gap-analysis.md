---
id: SR-15621
title: "Gap analysis — IR-276 CG replay equality index"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-276 AC-5, FR-033-AC-1, TC-048; src/replay/composite_builder.rs, tests/it/composite_original_eq.rs"
review_set: subset
---

## Summary

Reviewed the changed requirement, test, and production-code chain for the replay equality lookup. The added trace names FR-033-AC-1 and TC-048, and its Eq/Ne/mismatch controls exercise the public retained-context request path.

## Verdict

**PASS** — no gap found in this PR's replay lookup slice. IR-276 AC-5 remains broader than this PR and is not claimed complete here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

- Examined IR-276 AC-5 and FR-033-AC-1/TC-048. `#[trace("FR-033-AC-1", "TC-048")]` binds the new integration test to the actual operator criterion and test case.
- The new test's two successful requests assert different original node IDs; the third request asserts the exact requested and actual operators in `OperatorMismatch`. Removing the index would leave the behavioral test green, but the performance obligation is independently checked against the changed request code, where the per-request graph scan is absent.
- The constructor's one-time function search and the separately owned collection, capability, clause, and negotiation scans are outside this PR. This PASS concerns the named replay request path, not completion of all IR-276 work.
- No test gate ran during this independent review; the author reported its focused test separately and owns final-head qualification.

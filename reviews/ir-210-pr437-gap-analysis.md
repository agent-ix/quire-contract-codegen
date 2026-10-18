---
id: SR-12636
title: "IR-210 gap analysis of CG PR 437"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR 437, 3cfe03e1..a02f37ac; 16 oracle scalar test trace bindings and computed matrix"
review_set: subset
---

# IR-210 gap analysis of CG PR 437

## Summary

All 16 removed doc-tag ID lists remain on the same test functions in trace attributes. The two production doc tags stay in place; the diff adds no behavior or assertion change.

## Verdict

**PASS for the PR diff** — base and head coverage and normalized computed matrix are equal.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No new traceability gap in the reviewed diff (placeholder). | - |

## Coverage

Independent `quire coverage --strict` runs at base `3cfe03e1` and head `a02f37ac` each report 499/855 backed rows, 221 unbacked, and zero contradicted statuses. Computed TSV matrices each contain 813 lines including the header, or 812 criterion rows. Criterion IDs, statuses, statements and binder paths match after source line and column normalization. The strict exit status remains 1 because of the pre-existing unbacked rows. Plan completion: not assessed.

---
id: SR-11655
title: "IR-1072 PR 606 scope boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; spec/strategy/functional/FR-011-numeric-harness-campaigns.md; spec/strategy/matrix/TC-020-numeric-harness-campaigns.md"
review_set: subset
---

## Summary

Reviewed the new CG V2 entry boundary against the merged Contract IR accessor and V1 exclusion at frozen head ab7cb945c3c2c8a0c32b058ae8a2055709568035. The owner is carried once in the complete key, IR's refusal is preserved, and no V1 bridge is required. No applicable AssuranceProfile was found under spec/.

## Scope

- examined: FR-011 Inputs, Outputs, Behavior and AC-13 additions.
- examined: TC-020 step 12 and expected result.
- context_only: Contract IR clause_execution_point signature and MissingClaimOccurrence branch.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for allocation and V1/V2 boundary; the separate base and integrity artifacts record wording and test-evidence defects.

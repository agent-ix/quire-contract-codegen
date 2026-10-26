---
id: SR-584002
title: 'spec-review/base review of IR-917 CG PR #584'
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md,
  spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/kani/matrix/TC-025-bounded-kani-obligations.md
review_set: subset
---

## Summary

Reviewed CG PR #584 exact head 98fe6da149fc6992c2116a9207e14ecb06071e43, limited to FR-014, FR-015, TC-024 and TC-025 changes. QSL FR-093 was context. No applicable AssuranceProfile was found. The owner-admission and sibling-isolation test instructions conflict for reader-rejected packages.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-024 step 2 requires an unchanged generated sibling for malformed shapes that Contract IR’s strict reader rejects at package admission. Those cases produce no admitted package and thus no CG sibling oracle. Split the oracle-isolation assertion: for owner-rejected packages require the owner’s typed refusal and no CG generation; require unchanged sibling bytes only for an admitted or defensive-seam per-item refusal. | TC-024:305-310 |

## Verdict

PASS with one medium finding requiring an in-PR test-procedure correction.

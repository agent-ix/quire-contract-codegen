---
id: SR-8144
title: "IR-550 dependency spec review"
type: SpecReview
analysis: dependency
scope: "CG IR-550 six-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..9148d5b5a5d99350d2d5becba4761a2dd331c4d3"
review_set: subset
---

## Summary

The change depends on Contract IR V2 admission and lowering that exist on origin/main, not on QSL emission or the planned concrete owner path. No new relationship edge is introduced by the diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No dependency finding. IR FR-038-AC-96/101/107 and its tests establish admitted temporal and case forms; QSL-358/IR-349 remains separate from schedule-only generation. | CG FR-021-AC-31/32/33; IR origin/main FR-038-AC-96/101/107 |

## Final disposition

Rechecked exact local fix `47fe2d2afca6a3190478ed09adc5bfc32a65b1b3` against reviewed head `9148d5b5a5d99350d2d5becba4761a2dd331c4d3`. No finding to dispose in this method. The two-file FR-021/TC-031 change only clarifies the distinct valid call-item precondition for the body-root leg; it introduces no new issue under this lens. SR-8141 FND-001 is fixed.

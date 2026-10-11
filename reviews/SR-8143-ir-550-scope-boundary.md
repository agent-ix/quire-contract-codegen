---
id: SR-8143
title: "IR-550 scope-boundary spec review"
type: SpecReview
analysis: scope-boundary
scope: "CG IR-550 six-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..9148d5b5a5d99350d2d5becba4761a2dd331c4d3"
review_set: subset
---

## Summary

CG owns claim classification after an already admitted package and consumes Contract IR's public V2 reader/lowering records. QSL's current emission corpus identifies temporal formula, fairness, and expression case as not emitted; this spec properly uses IR-admitted fixtures instead of claiming current QSL emission.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional scope-boundary finding. Reader admission precedes CG generation; QSL emission is not an acceptance precondition here. | CG TC-024:96-108, TC-029:247-258, TC-031:121-136; QSL origin/main qsl-package/src/emit/tests/admission_corpus.rs:367-369, :403-405 |

## Final disposition

Rechecked exact local fix `47fe2d2afca6a3190478ed09adc5bfc32a65b1b3` against reviewed head `9148d5b5a5d99350d2d5becba4761a2dd331c4d3`. No finding to dispose in this method. The two-file FR-021/TC-031 change only clarifies the distinct valid call-item precondition for the body-root leg; it introduces no new issue under this lens. SR-8141 FND-001 is fixed.

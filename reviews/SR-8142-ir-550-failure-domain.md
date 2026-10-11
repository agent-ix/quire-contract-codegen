---
id: SR-8142
title: "IR-550 failure-domain spec review"
type: SpecReview
analysis: failure-domain
scope: "CG IR-550 six-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..9148d5b5a5d99350d2d5becba4761a2dd331c4d3"
review_set: subset
---

## Summary

Checked unsupported-child causality, lowered-case fallback, temporal-clause control, earlier duplicate/capability/lowering refusals, omitted claims, and sibling output isolation. The function-body masking case is recorded as SR-8141 FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No separate failure-domain finding. The new criteria call out causal ID versus requested ID, a case root that lowers, clause-versus-formula precedence, and absence of partial schedule/location output. | FR-014-AC-44; FR-018-AC-32; FR-021-AC-33; TC-024 step 21; TC-029 step 21; TC-031 step 13 |

## Final disposition

Rechecked exact local fix `47fe2d2afca6a3190478ed09adc5bfc32a65b1b3` against reviewed head `9148d5b5a5d99350d2d5becba4761a2dd331c4d3`. No finding to dispose in this method. The two-file FR-021/TC-031 change only clarifies the distinct valid call-item precondition for the body-root leg; it introduces no new issue under this lens. SR-8141 FND-001 is fixed.

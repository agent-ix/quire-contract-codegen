---
id: SR-8140
title: "IR-550 base spec review"
type: SpecReview
analysis: base
scope: "CG IR-550 six-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..9148d5b5a5d99350d2d5becba4761a2dd331c4d3"
review_set: subset
---

## Summary

Reviewed the three new direct-root criteria and their TC steps against the current CG generator entry points and Contract IR origin/main. Directly selected formula and fairness nodes are admissible within containing temporal clauses, while a case node can lower as data. QSL origin/main's emission corpus explicitly does not emit these three kinds, so the proposed Contract IR reader fixtures are the appropriate boundary. One function-body test precondition needs precision (SR-8141 FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional base-review finding beyond SR-8141 FND-001. Requested root ID and first unsupported reachable ID are correctly distinguished; the IR lowerer selects the latter in node-ID order. | CG FR-014-AC-44, FR-018-AC-32, FR-021-AC-33; IR origin/main crates/quire-contract-model/src/checked_package/v2/lower.rs:611-625 |

## Final disposition

Rechecked exact local fix `47fe2d2afca6a3190478ed09adc5bfc32a65b1b3` against reviewed head `9148d5b5a5d99350d2d5becba4761a2dd331c4d3`. No finding to dispose in this method. The two-file FR-021/TC-031 change only clarifies the distinct valid call-item precondition for the body-root leg; it introduces no new issue under this lens. SR-8141 FND-001 is fixed.

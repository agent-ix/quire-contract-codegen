---
id: SR-8806
title: IR-516 PR 377 evidence review
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-codegen; spec/routed/functional/FR-019-capability-settlement.md,
  spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md
review_set: subset
---

## Summary

Independent diff-scoped evidence review of CG PR 377, FR-019-AC-34 through AC-41, TC-030 and the routed matrix against QSL ADR-018/020/021/022/023/025/027 and CG AD-004. Ticket IR-516. 1 finding(s).

## Verdict

CONDITIONAL: medium-severity gap.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-030 compares reduction selections but does not assert that reduced and unreduced requests produce distinct obligation identities, as QSL ADR-021 RV-8 requires. A mutant that drops the reduction from identity could pass. | spec/routed/functional/FR-019-capability-settlement.md:396 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-39 and TC-030 now assert distinct obligation identities for selected symmetry/POR versus unreduced requests. |

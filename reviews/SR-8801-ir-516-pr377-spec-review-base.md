---
id: SR-8801
title: IR-516 PR 377 base review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/routed/functional/FR-019-capability-settlement.md,
  spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md
review_set: subset
---

## Summary

Independent diff-scoped base review of CG PR 377, FR-019-AC-34 through AC-41, TC-030 and the routed matrix against QSL ADR-018/020/021/022/023/025/027 and CG AD-004. Ticket IR-516. 1 finding(s).

## Verdict

FAIL: high-severity contract contradiction.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The blanket unadvertised-kind rule conflicts with FR-019 candidate validation: a supplied candidate lacking the item kind must settle invalid-request/inconsistent-candidates, not unsupported. | spec/routed/functional/FR-019-capability-settlement.md:179 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The candidate-kind precedence now explicitly retains invalid-request/inconsistent-candidates; TC-030 distinguishes the empty set from a supplied inconsistent candidate. |

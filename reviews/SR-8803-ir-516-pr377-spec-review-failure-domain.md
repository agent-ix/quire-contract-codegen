---
id: SR-8803
title: IR-516 PR 377 failure-domain review
type: SpecReview
analysis: failure-domain
scope: agent-ix/quire-contract-codegen; spec/routed/functional/FR-019-capability-settlement.md,
  spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md
review_set: subset
---

## Summary

Independent diff-scoped failure-domain review of CG PR 377, FR-019-AC-34 through AC-41, TC-030 and the routed matrix against QSL ADR-018/020/021/022/023/025/027 and CG AD-004. Ticket IR-516. 1 finding(s).

## Verdict

CONDITIONAL: medium-severity gap.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The sc branch of the weak-parallel test cannot be constructed: QSL ADR-025 MM-1 defines only tso and ra as weak. Test sc as a separate strong-model control. | spec/routed/functional/FR-019-capability-settlement.md:397 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Weak subjects are now tso/ra; sc is an explicit separate strong-model control in FR-019 and TC-030. |

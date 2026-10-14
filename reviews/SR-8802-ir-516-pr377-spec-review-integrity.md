---
id: SR-8802
title: IR-516 PR 377 integrity review
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen; spec/routed/functional/FR-019-capability-settlement.md,
  spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md
review_set: subset
---

## Summary

Independent diff-scoped integrity review of CG PR 377, FR-019-AC-34 through AC-41, TC-030 and the routed matrix against QSL ADR-018/020/021/022/023/025/027 and CG AD-004. Ticket IR-516. 1 finding(s).

## Verdict

FAIL: high-severity contract contradiction.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | RE-3 is an SMT unrolling method for refuting the existing whole TP-5 refinement record; the separately requested RE-3 refutation item specified here has no QSL requirement record or identity. | spec/routed/functional/FR-019-capability-settlement.md:393 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | AC-39 now places every selected reduction in the obligation identity, but QSL ADR-021 RD-1 includes state constraint among reductions and SC-3/RV-8 require it only in request identity. | spec/routed/functional/FR-019-capability-settlement.md:217 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The same whole TP-5 item now takes RE-3; only RE-2 per-step records are separately requested. |
| FND-002 | still-open | State constraint must remain a method parameter in request identity; narrow the obligation-identity claim to symmetry and POR. |
| FND-002 | fixed | FR-019 and TC-030 now distinguish symmetry/POR obligation identity from state-constraint request identity and prohibit cross-request result joins. |

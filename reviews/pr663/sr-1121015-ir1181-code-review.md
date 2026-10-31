---
id: SR-1121015
title: "IR-1181 PR 663 Code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/replay/matrix/tests.md"
review_set: subset
---

# IR-1181 PR 663 Code review

## Summary

Ticket: IR-1181. Exact head: 9856f90f6194790c65288596f0f906a926f9de33. TC-048 has one stale digest and retired-criterion procedure reference. The revised AC-19/20 obligations themselves preserve same-artifact and full report binding.

## Verdict

CHANGES REQUESTED for this method on the frozen PR diff. Reviewed FR-033-AC-6/9/19/20, TC-048, its matrix row, AD-003, and QSL main bcfc55fb.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-048 procedure still demands a canonical identity for the proved artifact and cites retired AC-9 twice. Replace the identity oracle with retained artifact/byte authentication and point the report-binding controls to AC-20; keep the same-artifact adverse cases under AC-19. | TC-048:51-55,219 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At `85671aedf095d232995f77b868ab1f4ee33189a2`, TC-048:51-56 retains proved artifact bytes and authenticates the observation under AC-19; the report-binding reference now names AC-20. TC-048:220 names AC-19/20 and adds artifact substitution as an adverse case. |

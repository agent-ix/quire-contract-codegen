---
id: SR-598010
title: "spec-review/integrity review of IR-1047 CG PR #600"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-041-claimed-module-proof-gate.md, spec/kani/matrix/TC-056-claimed-module-proof-gate.md, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md"
review_set: subset
---

## Summary

Reviewed CG PR #600 exact head 2c3722b62c42473f1a659f5b237d6b9855099cce against main 551731b53c555d52f058d7e57f282294e3c72cdc. Ticket: IR-1047. Reviewer model: GPT-6.1-Sol (gpt-6.1-sol). The V2 replacement transition contradicts the unchanged no-removed-claims rule.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-041-AC-9 replaces the held V1 row, but FR-041-AC-1 and Behavior require every merge-base claim to remain; the first V2 replacement must fail the census. Define a narrowly identified transition that consumes exactly the held V1 row only when the V2 control passes, while retaining rejection of every other removed claim. | FR-041-AC-9; FR-041-AC-1 |

## Verdict

FINDING: resolve before merge. Scope examined: FR-041-AC-6 through AC-9, TC-056 steps 6 through 8; FR-041-AC-1 through AC-5 and NFR-006-AC-19 were checked for preservation and consistency. Spec-only; no Cargo or Kani run.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | At c74c589b, AC-9 avoids the merge-base conflict by replacing its promised V1-row replacement with an additional V2 implication row. The ticket still requires replacement and hold expiry (Acceptance 4), and the existing V1 hold remains. This removes the required transition from the criterion instead of specifying a fail-closed same-claim transition. Restore the replacement obligation as a distinct Planned criterion and its narrow transition rule; a separate additive implication claim can remain separately specified. |
| FND-001 | fixed | At 6bf6a768, FR-041-AC-10 restores the same FR-003@4 claim continuity as a separate Planned criterion: the exact manifest/module identity stays, the row source fields update and held state clears only with a compiled same-claim V2 module and current-run installed-Kani healthy/mutant controls. FR-041-AC-1 still rejects removed and absent claims; AC-9 adds only a distinct implication claim and grants no FR-003@4 credit. IR-1047 Acceptance 4 remains unmet until implementation. |

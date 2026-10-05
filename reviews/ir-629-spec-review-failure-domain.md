---
id: SR-1569
title: "failure-domain review of IR-629 process-provider BackendKind spec (quire-contract-codegen#287)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@43fbf2f6083663e680e1ebeeeeefe11b288fdef5; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-046-process-provider-settlement.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---
# Failure-domain review: IR-629 process-provider BackendKind (PR #287)

## Summary

Ticket: IR-629. Looked for unstated failure modes, identity confusion and purity gaps in the
process-provider arm. Purity is stated (never reaches the plugin, FR-019-AC-13; its wording defect
is SR-1565 FND-001). Several plugins sharing one variant is handled by the existing candidate
table (more than one candidate and no named backend settles `ambiguous-backend`). A manifest that
advertises what the plugin cannot run surfaces after routing, in the arms open question 3 leaves
open. One identity-confusion mode is unstated.

## Verdict

Changes requested (one medium).

## Examined

- FR-019 bullets at lines 104 and 114, open questions 2 and 4 (examined)
- FR-019-AC-13 purity criterion and TC-046 step 3 (examined; wording finding in SR-1565)
- Multi-plugin settlement against the FR-019 candidate table (examined, clean)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No statement covers a plugin whose `BackendId` equals a first-party identity (`"kani"`) or the process-provider variant's own static identity. The driver maps every plugin `BackendId` to the process-provider variant, while CG's `from_identity("kani")` returns `Kani`, so such an item settles in the Kani arm (and in generation trips `BackendKindDisagrees` or runs as Kani). Neither the reserved identities nor which conversion wins is stated, and open questions 2 and 4 do not raise it | spec/routed/functional/FR-019-capability-settlement.md:104-105,114-115,128-135 |

## Dispositions

Round 1, reviewed at 48a7a96eca9d7dbc66a3c7a808f042151da2607c. New defects found this round are recorded in SR-1570.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 48a7a96 |

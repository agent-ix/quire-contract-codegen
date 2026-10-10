---
id: SR-4522
title: "Integrity review of PR 333 residual publication directives"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@8e07e731c4bab49a854f535dde49ec171d82a1a2; spec/kani/functional/FR-034-caller-death-ownership.md"
review_set: subset
---

## Summary

Checked the amended fixture obligations against the stage table, AC-23/24/28/34/37, and TC-049. One pre-existing sentence in the touched FR-034 file contradicts the revised rejection of cached gate-state certificates.

## Verdict

CONDITIONAL: correct the ClaimedGated confirming-observation cell before treating FR-034 as internally consistent. This finding does not claim the PR introduced the sentence.

## Examined scope

- FR-034 lines 223-245, 479-490, 568-700, 625-650, AC-23, AC-24, AC-28, AC-33, AC-34, AC-37: examined.
- TC-049 steps 2, 12, 14: context only; these reject cached prefix and publication as an oracle.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | ClaimedGated row says a cached prefix proves gate absence, contradicting the independent-observation rule | spec/kani/functional/FR-034-caller-death-ownership.md:485 | wrong-requirement |

The confirming-observation cell says, verbatim, "cached prefix proves no live gate." A run can select ClaimedGated, then have O release the gate or fail before C's fatal report. The selected prefix cannot establish whether the gate was retained or absent at that later boundary. FR-034 lines 581-588 and AC-23/28 reject that inference, and TC-049 step 2 does too. This sentence predates PR 333, but the one-file correction leaves it as an active oracle statement.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 3b0fe72964a923ba5c2348b6597ff1ae68cda29d: ClaimedGated now says "cached prefix establishes neither gate presence nor absence at report time." |

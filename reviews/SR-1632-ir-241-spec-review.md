---
id: SR-1632
title: "IR-241 TC-039 status change spec review (base)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen#295; spec/kani/matrix/TC-039-bounded-proof-ceilings.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-039
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---

# SR-1632: IR-241 TC-039 status change spec review (base)

## Summary

Ticket: IR-241. PR: quire-contract-codegen#295. The only spec change in the PR is the
`## Status` section of TC-039. It now reports partial delivery: steps 1 to 4 and the
process-tree part of step 19 are implemented, step 10 stays implemented, and the rest stays
planned. The review set is subset: base checklist plus integrity, which is SR-1633.

EARS conformance is excluded because no requirement statement or acceptance criterion changed.

## Method

The new Status text was checked against the base checklist:

- Accuracy of each claim against the code and tests at the reviewed candidate.
- That every one of the 14 named tests exists, each found once.
- That the shadow, refinement, strength, family, proof-subject and tool-version scope stays
  visibly planned.
- That no hash, digest, pin or tool-version tracking record was introduced. None was.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-039 Status says the process-tree part of FR-028-AC-21 is implemented and that its tests "kill that child and its sibling". It does not say that the procfs observer counts and kills only descendants seen at a poll. A descendant that leaves the group and is reparented before it is observed is neither counted nor killed (SR-1630 FND-002), so a reader takes the Status as whole-tree enforcement. | spec/kani/matrix/TC-039-bounded-proof-ceilings.md:171-176 |

## Verdict

**Approve after FND-001** (one sentence). The rest of the Status is accurate:

- Steps 1 to 4 are backed as listed.
- The bundle and corpus Kani-invoking tests are correctly disclaimed.
- Batching by equal ceilings is stated.
- Observed peak only, with no claim about memory between samples, matches the code.
- No limit-only mechanism is claimed.
- The remaining IR-241 scope is named explicitly and not claimed complete.

Suggested wording for FND-001: "The observer counts and kills the launcher group and every
descendant it has observed. A descendant that leaves the group and is reparented before the
first observation is not tracked."

## Dispositions

Round 1. The commit identity of each outcome is in the private tracker marker for this round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | PR #295 fix round: TC-039 Status now names mandatory PID-namespace ownership and its prerequisites, claimed init, startup-abort ordering, teardown on every conclusion, the escaped and late-fork fixtures, the conservative RSS metric, and the scope that is still planned. |

Round 3 (rebase regression; the exact head is in the private tracker marker): no regression in
this method's scope. Its examined spec and test paths carry the same patch as in round 2. The
rebased matrix keeps `main`'s FR-025-AC-9 row beside the PR's FR-028 rows, and the spec files
validate. The rebase's one compile regression is recorded as SR-1630 FND-014.

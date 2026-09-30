---
id: "SR-629"
title: "IR-93 slice 1 spec review: FR-017 precondition exception and capture bound, TC-027"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@c5cdf7d396dc1ca1ba48c8b164f9149827d3030c; spec/functional/complete-v1/FR-017-kani-execution-evidence.md, spec/test/complete-v1/TC-027-kani-execution-evidence.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-027
    type: reviews
---

# SR-629: IR-93 slice 1 spec review

## Summary

Ticket: IR-93 (slice 1: IR-310, IR-353). PR: agent-ix/quire-contract-codegen#202, head c5cdf7d.
Method: spec-review, with the EARS and integrity checks folded in. `make spec` (quire validate)
passes and raises no warning on FR-017 or TC-027.

## Method

I read the three FR-017 edits and the TC-027 edit against the rest of FR-017's Behavior section
and against the code that implements them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-017 still says that the outcome classification step, among others, does not branch "on which kind it is". The PR makes classification branch on `ObligationKind::Precondition`. "Kind" there means harness kind (contract versus exact-scalar), but after this change two readers can reasonably disagree on whether the new branch violates it. | spec/functional/complete-v1/FR-017-kani-execution-evidence.md:88-92, spec/functional/complete-v1/FR-017-kani-execution-evidence.md:75-78 |
| FND-002 | low | The exception clause "except for a precondition harness, whose only property is its non-vacuity cover and which the cover alone decides" has no `shall` and names no outcome. The reader has to infer that the verified, cover-unsatisfied and missing-cover-summary rules apply. | spec/functional/complete-v1/FR-017-kani-execution-evidence.md:75-78 |
| FND-003 | low | The 8 MiB tail bound on captured stdout and stderr is normative but appears only as an Inputs bullet. There is no Behavior statement or AC, so TC-027 cannot trace it. | spec/functional/complete-v1/FR-017-kani-execution-evidence.md:41-42 |

## Verdict

Approve after FND-001, which needs one clause. For example: "none of those steps branches on the
harness kind (contract or exact-scalar); classification reads the obligation kind only to apply
the precondition exception."

For FND-002, suggested wording: "…, except that for a precondition harness the generator shall
skip this rule and classify the run by its cover summary alone."

FR-017-AC-13 and the TC-027 Expected Results agree with each other and with the code. The
exception is sound: a precondition harness asserts nothing beyond its cover.

## New findings (disposition pass 1)

Reviewed at 5ef88a8a75a755f777e9ff774534d17748979fd1.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | FR-017-AC-14 is compound. It bundles four independent behaviours in one criterion: the 8 MiB tail, a `Duration::MAX` timeout, stop-and-join of the capture threads, and the group kill. So one failing clause fails the whole AC, and the matrix cannot mark a clause partial (the group-kill clause is Linux-only in test). | spec/functional/complete-v1/FR-017-kani-execution-evidence.md:134 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 4af361b480a0895203801d65d6fe1c2a8dd9b2a9: "none of those steps branches on which kind it is, except that the classification applies the zero-checks rule to every harness but a precondition harness." |
| FND-002 | fixed | 4af361b480a0895203801d65d6fe1c2a8dd9b2a9: "for a precondition harness … the generator shall instead classify the run by its cover summary alone, as verified, cover-unsatisfied or inconclusive under the cover rules above." |
| FND-003 | fixed | 4af361b480a0895203801d65d6fe1c2a8dd9b2a9: a Behavior statement ("shall keep at most the last 8 MiB …") and FR-017-AC-14. |

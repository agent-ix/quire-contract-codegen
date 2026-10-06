---
id: SR-1944
title: "IR-652 lifecycle spec review (ears-conformance)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@6b9cbd21f2db16abc6e6f777f11a372b689c9080; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-034-caller-death-ownership.md (frozen candidate diff against main f4c37b253250671fdd990a635da963d013b5478c; PR not open)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
---

# SR-1944: IR-652 lifecycle spec review, ears-conformance

## Summary

Ticket: IR-652. The PR is not open; this reviews frozen candidate 6b9cbd21f2db16abc6e6f777f11a372b689c9080.
Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run
5066cb1a-6ce7-4ef1-9919-36e3a3ff45a7. EARS grammar of the new and changed requirement
statements. Two low findings.

## Method

Ran `quire validate` on both FR files. Its EARS checker flags only unchanged FR-017 line 167, so
the new bullets were classified by hand. Scope units examined:
- The four new FR-017 Behavior bullets (lines 96 to 116).
- FR-034 new "shall" statements at lines 74 to 84, 109 and 110, 149 and 150, and 151 and 152.

"L shall arm its actual C parent-death signal ..." is an event-ordered ubiquitous statement with a
subject. "The bounded executor shall launch only trusted first-party O and I ..." and "The live
owner shall reap its actual direct child ..." are ubiquitous and conform.

## Verdict

**PASS with two low findings.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two FR-017 Behavior bullets state obligations without a subject or "shall": "The report is internal per-run unnamed kernel storage ..." and "Authenticated Completed precedes original-lease closure ...". Every other bullet in the list reads "The generator shall ...". Both are unclassifiable as EARS. Rewrite them as "The generator shall ..." statements, using the event or state pattern for the ordering. | spec/kani/functional/FR-017-kani-execution-evidence.md:101-105, spec/kani/functional/FR-017-kani-execution-evidence.md:111-116 |
| FND-002 | low | "Report lifetime shall use unnamed kernel storage; other assigned artifacts retain explicit cleanup owners." The subject is a property, not a system or actor, and a second clause without "shall" is joined to it. Split it into two statements with system subjects. | spec/kani/functional/FR-034-caller-death-ownership.md:149-150 |

## Dispositions

Round 1 reviewed a661f2f297f5e9860c25e9995226d07332426ab2 (previous 6b9cbd21f2db16abc6e6f777f11a372b689c9080; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 9a4f4275-0799-4c05-867e-b01504c3095c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: FR-017 bullets now read "The generator shall" / "When ..., the generator shall". |
| FND-002 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Split into two statements with system subjects. |

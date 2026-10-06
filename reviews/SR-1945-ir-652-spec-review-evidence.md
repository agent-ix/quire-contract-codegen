---
id: SR-1945
title: "IR-652 lifecycle spec review (evidence)"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@6b9cbd21f2db16abc6e6f777f11a372b689c9080; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-049-caller-death-ownership.md (frozen candidate diff against main f4c37b253250671fdd990a635da963d013b5478c; PR not open)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1945: IR-652 lifecycle spec review, evidence

## Summary

Ticket: IR-652. The PR is not open; this reviews frozen candidate 6b9cbd21f2db16abc6e6f777f11a372b689c9080.
Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run
5066cb1a-6ce7-4ef1-9919-36e3a3ff45a7. Declared verification method per changed criterion against
the evidence TC-027 and TC-049 prescribe. One medium and one low finding.

## Method

Matched the Verification column of FR-017-AC-19 and FR-034-AC-2, 5, 7, 12, 14, 17, 23, 26 and 31
to 34 with TC-027 Unnamed report storage and TC-049 steps 5, 7, 15 and 16. A runtime refusal
counts as Test; reading documentation counts as Inspection. All of these were examined. Clean:
- AC-2, 5, 7, 12, 17 and 34 are Test and have runtime steps.
- FR-017-AC-19 is Test (TC-027).
- Both TCs claim no executed evidence.

## Verdict

**FAIL: one medium and one low finding.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034-AC-14 declares Inspection only, but it now carries runtime behaviour: "The compiled caller security profile is checked ... Missing capability gives typed pre-Dispatch refusal without host-policy mutation or weaker mode." TC-049 step 5 exercises that refusal at runtime. Inspection of the setup docs cannot fail on it. Declare Test and Inspection, or move the runtime clauses into a Test criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:516, spec/kani/matrix/TC-049-caller-death-ownership.md:104-108 |
| FND-002 | low | FR-034-AC-31, 32 and 33 declare "Test, Analysis" without naming what the Analysis is or where it is recorded. The only Analysis text is the primary-source grounding paragraph in Dependencies, which says it is not production evidence. Name the Analysis artifact, or drop the method. | spec/kani/functional/FR-034-caller-death-ownership.md:534-536, spec/kani/functional/FR-034-caller-death-ownership.md:557-567 |

## Dispositions

Round 1 reviewed a661f2f297f5e9860c25e9995226d07332426ab2 (previous 6b9cbd21f2db16abc6e6f777f11a372b689c9080; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 9a4f4275-0799-4c05-867e-b01504c3095c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: FR-034-AC-14 declares Test, Inspection. |
| FND-002 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: FR-034-AC-31, 32 and 33 declare Test only. |

## Round 3 scoped delta

Round 3 scoped delta reviewed 0de3e8823f0cae6382fd8d465fd0a787d5109b35 on branch ir652-lifecycle-spec (fresh main fcf7f6a415a31a80824eafbe95b64bf977555c38; normative rebased equivalent a54cd1c1b880947487bee7c2e29382366318a03b of 6f552cd97c8a6915d999e03d49129ebc3c198195, the four normative files byte-identical across the rebase; PR not open). Scope: only the evidence-allocation and one-PR sequencing delta a54cd1c1b880947487bee7c2e29382366318a03b..0de3e8823f0cae6382fd8d465fd0a787d5109b35 in FR-034 (Dependencies-adjacent staging paragraph), TC-027 (staging paragraph) and TC-049 (Evidence delivery allocation); no criterion row, id, Trace, Rust, test or review artifact changed. Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 0f55a2f0-235f-4999-b03a-1a17c640a3df. Earlier interrupted attempt at 1f81f04 produced no verdict and no records. All prior findings keep their latest outcome; no disposition row is added.

**Round 3 verdict: clean for the scoped delta.** The allocation keeps Test as the required method: inspection, source schedule analysis and scratch facilities are excluded as substitutes, external pins count only for observed boundaries, and AC-14's Test, Inspection split is respected.

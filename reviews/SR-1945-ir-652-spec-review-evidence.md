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

---
id: SR-1765
title: "IR-639 guardian-test-support spec review (base, manual)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@7742a43f62fc3feddde61a529a2bab37b4d0a67b; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md (PR #305 diff against main 28553daeb1e9cfd88bb6620125bb1df27273c68e)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1765: IR-639 guardian-test-support spec review, base (manual)

## Summary

Ticket: IR-639. PR: quire-contract-codegen#305. Reviewer: claude-opus-5-5, session
9072f908-e626-4176-ae33-69e582990445. Jev is not installed on this machine (`command -v jev`
finds nothing), so this analysis is a manual judgement and not a calibrated Jev run. I judged
whether each new or changed criterion can fail and whether its oracle is defined well enough for
two implementers to build the same check. Two findings: one medium and one low.

## Method

For FR-034-AC-23, AC-24 and AC-27 to AC-30, I asked what observable result distinguishes a
correct implementation from the mutants named in TC-049 step 12, and whether the criterion
defines that result or only names it. Scope units examined: FR-034-AC-23, AC-24, AC-27, AC-28,
AC-29 and AC-30, and TC-049 steps 12 to 14.

## Verdict

**FAIL: one medium finding and one low finding.** The following can fail:
- AC-24: an ignored-EOF mutant records escalation-required, which is a failed oracle even after
  cleanup kills the worker.
- AC-27: a default-on feature or an exported handle fails.
- AC-29: a feature-off helper that is accepted fails.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034-AC-28 requires the fixture operation to "seal the required ownership oracle", and the FR body says it "evaluates and seals the named ownership oracle". Neither defines the oracle's predicate or its inputs. The assertions are only named in TC-049 step 12 (surviving-descendant, closed-lease authorization/premature-marker). One implementer would compute a pass/fail verdict inside the feature-gated library, which puts test logic into production code. Another would return raw observations for the test to judge. Define the oracle's inputs and outcome per scenario (post-Dispatch: confirmed guardian termination with the worker dead before escalation; pre-Dispatch: no backend marker and confirmed termination). State which side evaluates it. | spec/kani/functional/FR-034-caller-death-ownership.md:199-201, spec/kani/functional/FR-034-caller-death-ownership.md:271 |
| FND-002 | low | FR-034-AC-30 ends with "Existing assertion delivery is not claimed.", which is status prose that no test can pass or fail. Delivery status belongs in the matrix Status column, which already marks FR-034 as Planned. Remove the sentence from the criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:273 |

## Method availability correction

Disposition pass 1. This artifact was first labelled `criterion-strength`. The installed
`spec-criterion-strength-analysis` skill requires the Jev client and must not run without it. Jev
is not installed here, so that calibrated method was unavailable and never ran. No Jev score,
calibration or weakness_kind was produced. The two findings above came from a manual reading of
criterion clarity and testability. They are reclassified unchanged as a general base spec review
(`analysis: base`). FND rows, text and severities are unchanged.

## Dispositions

Round 1, reviewed at ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: The library now returns sealed raw observations and evaluates no oracle. The external harness owns the AC-24 predicate, defined per scenario from pre-escalation observations. |
| FND-002 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: The status sentence is removed from AC-30. Planned and gated status now lives in the matrix Status column. |

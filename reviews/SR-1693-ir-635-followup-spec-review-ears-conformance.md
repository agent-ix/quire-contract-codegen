---
id: SR-1693
title: "IR-635 follow-up spec review (EARS): edited FR-033 and FR-029 requirement statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@04608d2942dc27f47d0b820a8a58a1b82b1669c3; spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

# SR-1693: IR-635 follow-up spec review, EARS conformance

## Summary

Ticket: IR-635. PR: quire-contract-codegen#303 (spec only). Reviewer: claude-opus-5-5. I checked the requirement statements this PR added or edited for EARS form. One low finding.

## Method

I checked the following:

- **FR-033:** each new or edited Behavior bullet (the Refinement projection, the claim-validity refusal, the falsified and verified consumption, the coverage bullet) and the Falsified Settlement lead sentence
- **FR-029:** the edited falsified paragraph and the common-admission sentence

For each one I checked for a single responsible actor, a `shall`, an explicit trigger or state where one applies, and one obligation per statement. Acceptance criteria and AD-003 architecture prose are out of EARS scope.

## Verdict

**PASS with one low finding.** These statements are EARS-conformant, each with an actor, a `shall` and one obligation:

- the Refinement projection bullet
- the claim-validity bullet
- the falsified and verified consumption bullets
- the Falsified Settlement lead ("When … admission succeeds, CG shall preserve …")
- FR-029's "When common admission succeeds, the adapter shall apply refinement disagreement …"

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The edited coverage Behavior bullet states the builder and converter obligations with `shall`. It then appends five more obligations as bare declaratives with no actor or `shall`: "Missing keys stay uncovered …", "Literal operands have singleton domains …", "Full enum coverage requires every source-declared variant", "Other non-Boolean leaf families … stay uncovered/Tested …", and the TX-3 sentence. It is a compound requirement, so a reviewer cannot tell which component must enforce the enum and leaf-family rules. Split the bullet into one `shall` statement per rule, each naming its actor: the converter consumes QSL's coverage. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:153 |

## Dispositions

Round 1 re-check of fix commit `48f3555` on quire-contract-codegen#303. Each finding was verified against the spec text at that commit, not against the author's receipt.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 48f3555: The compound coverage bullet is split into nine single-obligation statements, each with a named builder or converter actor and shall. |

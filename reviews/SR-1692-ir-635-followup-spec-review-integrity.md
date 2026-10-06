---
id: SR-1692
title: "IR-635 follow-up spec review (integrity): disagreement precedence versus CG setup refusals"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@04608d2942dc27f47d0b820a8a58a1b82b1669c3; spec/assurance/AD-003-evidence-chain.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/replay/matrix/tests.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

# SR-1692: IR-635 follow-up spec review, integrity

## Summary

Ticket: IR-635. PR: quire-contract-codegen#303 (spec only). Reviewer: claude-opus-5-5. I checked the changed artifacts for internal contradictions, unique ids, matrix and range consistency, and relationship symmetry. One high finding.

## Method

I cross-read four things against FR-033's unchanged Setup Refusal Precedence, its AC-3 and its Outputs:

- the new FR-033 converter bullet, the Falsified Settlement table, and AC-11, AC-12 and AC-13
- FR-029's falsified paragraph and AC-20/24
- TC-048 steps 9 and 10
- AD-003's E-1 composite extension

I also checked:

- that the matrix ranges agree across all four index files: replay `tests.md`, the TC-048 summary, the root `tests.md`, and the kani `tests.md`
- that the AD-003→FR-033 and FR-033→AD-003 `references` edges exist
- that AC-11, AC-12 and AC-13 are new, unique ids

## Verdict

**FAIL on one high finding.** The following are consistent:

- the ranges FR-033-AC-1 to AC-13 in every matrix row and the TC-048 summary
- the relationship edges in both directions
- the FR-028-AC-2/3 wording against FR-029's ordinary map
- AD-003's scope and E-4 restatement against FR-029-AC-19
- the TC-048 Expected Results rows for AC-11/12/13

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new precedence lets only claim, source, node, occurrence, bounds and O-09 refusals come before `Disagreed`. For a valid claim, FR-033-AC-12 and F-1 make `Disagreed` win over operand refusal and native fault. FR-029-AC-24 makes it `Failed`/`CgDefect` for a falsified outcome, and FR-029 Behavior says "The disagreement record is never lost to another path". FR-033's unchanged Setup Refusal Precedence and AC-3 still put two refusals before the QSL call: CG-side canonical and declared-domain validation (step 3, out-of-domain leaf), and missing native observation (step 5). Each is "a typed setup refusal before QSL evaluation" with no settlement. Example: the measured FR-358-AC-8 case `x: 12` with `refinement: Disagreed`. CG's setup refuses it, so F-1 is never reached and the retained disagreement is lost. TC-048 step 10 ("combine Disagreed with operand refusal") cannot pass against the setup list. Split the setup list into claim-identity refusals, which precede `Disagreed`, and operand and native-observation refusals. For the second group, state whether a retained `Disagreed` still yields `Failed`/`CgDefect` (as AC-24 requires) or why it does not. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:197, spec/replay/functional/FR-033-composite-parity-replay-binding.md:202, spec/replay/functional/FR-033-composite-parity-replay-binding.md:205, spec/replay/functional/FR-033-composite-parity-replay-binding.md:231, spec/kani/functional/FR-029-run-outcome-terminal-record.md:182, spec/kani/functional/FR-029-run-outcome-terminal-record.md:317 |

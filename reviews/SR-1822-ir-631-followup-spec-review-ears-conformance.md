---
id: SR-1822
title: "IR-631 follow-up EARS conformance: FR-032 scalar identity statements and criteria"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8; spec/replay/functional/FR-032-routed-scalar-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

# SR-1822: IR-631 follow-up EARS conformance

## Summary

Ticket: IR-631. PR: quire-contract-codegen, not yet opened. It is spec only. Reviewer: claude-opus-5-5, session 8950150c-5937-457d-9408-c19aa15729b8, run f92b6bc1-4101-4d05-affe-ea238bac36af. One low finding.

## Method

I checked the changed FR-032 statements for EARS form: the Behavior bullet on full-claim comparison, the "Scalar Obligation Identity" section and the terminal paragraph. I also checked new and amended criteria AC-1 and AC-3 to AC-10 for a single trigger and response. Unchanged statements were read for context only.

## Verdict

**PASS with one low finding.** The changed Behavior bullet is event-driven: "When asked to return a settlement or derive its terminal value, the converter shall compare…". The identity section uses "CG shall" for each obligation it imposes. AC-6, AC-7 and AC-9 are long but describe one subject each.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-10 combines at least eight independently failing obligations in one criterion:<br>1. byte-exact canonical conformance<br>2. QSL acceptance of the CG-minted identity<br>3. a digest-mismatch refusal with a named cause and codes<br>4. mutated operand, occurrence or kind refusals<br>5. a ban on copied verifiers or synthetic success<br>6. invented occurrence or operand and inline-literal refusals<br>7. encoding refusal with no identity<br>8. unchanged function, clause and frame behaviour<br>A test that covers only some of them has no partial status, and a matrix row cannot say which part is covered. Split AC-10 into separate criteria for conformance and acceptance, mismatch refusal, membership refusal, encoding refusal, and the unchanged other paths. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:301 |

## New findings (disposition pass 1)

Found while re-checking the fix at `832633d7afa778e8a3688601595beb5d38917cb5` (round 1, run 733cf463-44a3-4ff9-83f7-a1a0a32f9463). It is a regression introduced by the fix for FND-001, not part of a fresh sweep.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | Splitting AC-10 gave AC-11 to AC-14 only the label 'PLANNED (IR-631).', the same label as AC-2, the one criterion described as implementable independently. AC-1 and AC-3 to AC-10 carry 'CODE-GATED (...)', and the Prerequisites paragraph says 'AC-4 through AC-14's completed scalar route are additionally Gated'. AC-11 and AC-12 need the actual QSL route with a CG-minted claim, and AC-13 needs the CG builder's setup refusal; both depend on the gated scalar-kind and metadata retention. A reader of the criteria table can take AC-11 to AC-14 as ungated like AC-2. Give them the same CODE-GATED label, naming their gate. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:389-392, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:476-477 |

## Dispositions

Round 1 re-check of fix commit `832633d7afa778e8a3688601595beb5d38917cb5` (run 733cf463-44a3-4ff9-83f7-a1a0a32f9463, model claude-opus-5-5). Each finding was verified against the spec text at that commit, not against the author's receipt. The reviewed content of `f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8` was confirmed unchanged after the rebase onto main. The planned criteria remain unrun; no implementation, mutation coverage or settlement proof is claimed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 832633d7afa778e8a3688601595beb5d38917cb5: AC-10 is split into AC-10 (conformance and acceptance), AC-11 (digest mismatch), AC-12 (membership refusal), AC-13 (encoding refusal) and AC-14 (other paths unchanged), each with its own Expected Results row. |

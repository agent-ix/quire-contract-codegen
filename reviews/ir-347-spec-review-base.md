---
id: SR-17006
title: "IR-347 CG #562 spec review (AD-003/AD-004 arithmetic retirement reconciliation)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#562; spec/assurance/AD-003-evidence-chain.md; spec/assurance/AD-004-cg-crate-layout.md"
review_set: subset
---

## Summary

Ticket: IR-347 (paired with IR-348). Independent spec review of the diff at exact head `036539b64a2fd89f99365f80fc2802447ee33e2a` against `origin/main` `9a0dc2bb1ad34e6b360e73f0d11b9fee9cbc31e7`. The PR reconciles AD-003 and AD-004 with FR-015-AC-109, which is already Implemented, and with the merged arithmetic retirement. Older text still prescribed three CG family lowerers. The diff changes no FR, AC, TC or matrix file. FR-015-AC-109 is unchanged and verbatim. FR-015-AC-110 and FR-015-AC-111 remain PLANNED. IR FR-039 is not referenced or altered. The renderer-row change from `KaniOutcome` to `BoundedCorpusExpectation` corrects text that predates the change and matches FR-015-AC-91 and AC-52.

## Examined criteria

- FR-015-AC-109 (context_only): Implemented arithmetic slice; deletes the `Arithmetic` arm and `lower/definedness_arithmetic.rs`.
- FR-015-AC-110 (context_only): PLANNED collection slice.
- FR-015-AC-111 (context_only): PLANNED graph and final slice.
- FR-015-AC-91 (context_only): interim path returns `BoundedCorpusExpectation`, not a `KaniOutcome`.
- FR-015-AC-95 (context_only): end state, PLANNED.
- AD-003 seam bullet (examined).
- AD-004 layout comment, module-map row, renderer row, step 4g closing sentences, interim-exceptions row and item-map bullet (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AD-004 step 4g states arithmetic retirement twice in adjacent sentences: the new "FR-015-AC-109 has retired CG's arithmetic lowerer, so the current CG lowerer set is collection and graph" is followed by the existing "Arithmetic retirement (FR-015-AC-109) is implemented" | spec/assurance/AD-004-cg-crate-layout.md:1105-1108 |
| FND-002 | low | The Interim exceptions table lists places where the AD target is not yet true. Its rewritten row is now titled with a past state ("The semantic-family forwarding modules at step 2f") that the row itself says IR-347 replaced, so the exception that remains (CG collection and graph finite lowerers pending AC-110/AC-111, and pending IR export removal) is not what the Exception cell names | spec/assurance/AD-004-cg-crate-layout.md:1412 |

## Verdict

**PASS with two low findings.** Neither blocks the merge. The narrow correction is accurate. Remaining work stays visible: IR export removal, AC-110, AC-111, step 4g incomplete and AC-95 PLANNED. No acceptance criterion is narrowed and no new implementation claim is made. Observation outside the diff, not a finding: the unchanged AD-004 step 4g text at line 1092 still says the graph slice removes "the generation-time `KaniOutcome`". FR-015-AC-111 describes this as the generation-time expectation plus the typed refusal `KaniOutcome`.

## Dispositions

Round 1, verified against fix commit `d0727fe87f4dad1f077d50d37ea4b2092b0b1c26` (parent `036539b6`), which is the PR head. The fix changes only these two places in `spec/assurance/AD-004-cg-crate-layout.md` and adds the three reviewer artifacts under `reviews/`, each byte-identical to the original. The fix introduces no regression. FR-015-AC-109, AC-110 and AC-111 and all other FR/AC text are untouched.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The repeated sentence "Arithmetic retirement (FR-015-AC-109) is implemented." is removed; the retirement is stated once (AD-004:1106-1107) |
| FND-002 | fixed | The interim-exception row is now titled "The remaining collection and graph finite lowerers", names the current files and keeps the IR export removal and AC-110/AC-111 work (AD-004:1411) |

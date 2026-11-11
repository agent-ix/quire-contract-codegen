---
id: SR-17940486709564847047690
title: "IR-1516 PR #790 two-label identity and obligation-identity cross-check: evidence"
type: SpecReview
analysis: evidence
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #790 @ b0763d44; IR-1516; spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-1516. I checked the verification method (Test, TC-035) for AC-65/66 and whether each TC-035 step 46 mutation control can fail. For each control I asked three questions: is there a distinguishing input, is there a named failing assertion, and is the mutant site named? The site is named as "the identity comparison in `InvariantReplayInputs::decode_falsification`". The distinguishing input is a caller identity that differs only in its ordinal, buildable with `ObligationIdentity::new(node, model, ordinal + 1)` (qsl-replay/src/identity.rs:75).

- Mutant 1, ignore the caller identity: returns `Ok` on the ordinal case and fails the `ObligationIdentityMismatch` assertion. Sound.
- Mutant 2, compare only part of the identity: sound in intent, but named over members `ObligationIdentity` does not have (FND-001).
- Mutant 3, map to `HarnessMismatch`: fails the "not `HarnessMismatch`" variant assertion. Sound.
- Mutant 4, overwrite the caller identity before comparing: the harness's-own branch never refuses and fails the mismatch assertion, which is sound. The "default" branch is mispredicted (FND-002).

Test is the right method for both ACs. The equal-identity accept case of AC-66 is the positive control that kills an always-refuse mutant.

## Verdict

PASS WITH FINDINGS. Every control can fail. Two of them are worded so that the predicted member or assertion is wrong.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Step 46's second mutant "compares only the identity's kind or name". QSL `ObligationIdentity` is `{node: NodeKey ([u8; 32]), model: Option<ModelIdentity>, ordinal: u32}` and has no kind or name member (qsl-replay/src/identity.rs:61-71). Reword to "compares only the node and model". | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:401-402 |
| FND-002 | low | Step 46's fourth mutant: "replaces the caller's identity with a default or the harness's own before comparing never refuses and fails the same assertion". That holds for the harness's-own branch only. A default identity differs from the harness's in both the equal and the ordinal case, so that mutant refuses both. It passes the `ObligationIdentityMismatch` assertion and is killed by the AC-66 accept assertion instead. Name the accept assertion for the default branch, or split the control. | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:403-405 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| TC-035-step-46 | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:394-405 | Over the step 45 invariant harness, supply a caller `obligation_identity` equal to the harness's `obligation` and assert `decode_falsification` accepts it ... Then supply an identity that differs from the harness's only in its ordinal and assert `ObligationIdentityMismatch` ... Mutation controls: ... The refusal site is the identity comparison in `InvariantReplayInputs::decode_falsification`. |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | ... over one invariant harness, one caller identity differing from the harness's only in its ordinal returns `ObligationIdentityMismatch` and not `HarnessMismatch`. \| Test (TC-035) |
| FR-024-AC-66 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 | When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it ... \| Test (TC-035) |

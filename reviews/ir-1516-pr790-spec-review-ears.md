---
id: SR-17940486709564847047688
title: "IR-1516 PR #790 two-label identity and obligation-identity cross-check: EARS conformance"
type: SpecReview
analysis: ears-conformance
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #790 @ b0763d44; IR-1516; spec/replay/functional/FR-024-counterexample-envelope-intake.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-1516. I checked the changed requirement statements and criteria of FR-024 against EARS.

- The new refusal is added as one more item of the existing unwanted-behavior bullet ("If the harness is not a V2 invariant harness, then the generator shall return ...; if ... `ObligationIdentityMismatch` (FR-024-AC-65)"). It adds no second `shall`, and the bullet already holds a list in that form.
- AC-65 and AC-66 are event-shaped verification statements ("When ... receives ..., it returns ..."; "When ... equals ..., `decode_falsification` accepts it"). Each has one obligation subject (`decode_falsification`).

`make spec` reports the same 6 warnings as origin/main, none of them in a changed file. `quire validate` on FR-024, TC-035 and AD-002 exits 0.

## Verdict

PASS. No EARS defect. Each new or changed statement has one subject and one response.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-behavior-invariant-refusals | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:449-458 | If the harness is not a V2 invariant harness, then the generator shall return `StateClauseReplayError::NotInvariantHarness`; ... if the harness's `ObligationIdentity` differs from the caller's, `ObligationIdentityMismatch` (FR-024-AC-65); ... each without calling the executor. |
| FR-024-behavior-invocation | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:411-414 | The generator shall build the invocation document with `format` `quire.state.invocation/v1`, ... `pre` and `post` each the snapshot's two identity labels (authority and identity) ... |
| FR-024-behavior-encode | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:462-466 | Each state document declares its own two identity labels; QSL reads those labels from the document bytes. ... If two documents ... declare the same two identity labels, then the generator shall return `StateClauseReplayError::DuplicateDocumentIdentity` ... |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | When `InvariantReplayInputs::decode_falsification` receives a V2 invariant harness whose `obligation` ... differs in any member from the `obligation_identity` the caller put in the inputs, it returns the distinct typed `StateClauseReplayError::ObligationIdentityMismatch` ... |
| FR-024-AC-66 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 | When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it, and the packet and the request that `StateClauseReplay::new_invariant` builds carry that identity unchanged. |

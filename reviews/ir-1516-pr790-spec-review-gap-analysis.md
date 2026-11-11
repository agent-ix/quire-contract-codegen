---
id: SR-17940486709564847047691
title: "IR-1516 PR #790 two-label identity and obligation-identity cross-check: gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #790 @ b0763d44; IR-1516; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md; src/replay/state_clause.rs, src/replay/state_clause/invariant.rs, src/kani/identity.rs (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-1516. Plan completion: not assessed. The PR is spec-only, so there is no production code to trace. I compared the spec scope with IR-1516's ask. Ask (2) is that "StateClauseReplay::new and new_invariant take the caller's obligation_identity and never check it against the harness ... cross-check it and refuse a mismatch". The PR specs `new_invariant` only (AC-65/66).

The postcondition path cannot carry the check today:

- `StateClauseReplayInputs::decode_falsification` (src/replay/state_clause.rs:456) takes `&StateFrameIdentity`.
- Neither `StateFrameIdentity` (src/kani/identity.rs:1074-1111) nor `StateFrameHarness` (:994-1000) carries a QSL `ObligationIdentity`.
- `StateClauseReplay::new` therefore has nothing to compare `obligation_identity` against.

Only the V2 clause harness carries one (`KaniV2ClauseObligationHarness.obligation`, :489).

The diff and the PR body do not record this gap. IR-1519's description notes it, but nothing in the spec does.

`quire matrix`: AC-65/66 untagged (planned). `make spec`: 6 warnings, the same as origin/main.

## Verdict

PASS WITH FINDINGS.

Ruling on the postcondition gap: option (ii). The gap is acceptable in this PR with a named follow-up, because speccing a check needs a design decision first: there is no seam to specify. Two conditions apply:

- File the follow-up ticket before merge and name it in FR-024. A one-line note in the state-clause Scope or under AC-65 is enough, for example "the postcondition path's identity cross-check is IR-XXXX". Without that, the asymmetry is silent.
- Do not close IR-1516 as having delivered `new`.

The follow-up must decide four things:

- Where the producer-issued `ObligationIdentity` lives on the operation-contract harness: a `StateFrameHarness` member sealed by provenance, like `KaniV2ClauseObligationHarness`, or a member of the persisted `StateFrameIdentity` record. The record's doc says `state_fields` "is no member of an obligation identity", so this needs an explicit ruling.
- How a harness that covers both a frame and a postcondition obligation of one `state_clause` (`StateFrameIdentity.clause`: "both obligations of a request") carries one identity per property.
- The refusal variant and its settlement on the `new` path.
- Whether migrating postconditions to the V2 clause harness supersedes the work.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-024-AC-65 specs the obligation-identity cross-check for `new_invariant` only. IR-1516 also asks for `StateClauseReplay::new` (postcondition), which still trusts the caller's `obligation_identity` unchecked, and the spec does not record this. The frame harness (`StateFrameIdentity`/`StateFrameHarness`) carries no `ObligationIdentity`, so there is no seam yet. Ruling (ii): acceptable with a named follow-up ticket filed before merge and cited in FR-024 (Scope or beside AC-65). The ticket should decide where the identity lives on the frame harness, how one identity per property works for a frame+postcondition harness, and the refusal variant and settlement. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595; src/replay/state_clause.rs:456-461; src/kani/identity.rs:994-1111 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | When `InvariantReplayInputs::decode_falsification` receives a V2 invariant harness whose `obligation` (QSL's `ObligationIdentity`) differs in any member from the `obligation_identity` the caller put in the inputs, it returns the distinct typed `StateClauseReplayError::ObligationIdentityMismatch` ... |
| FR-024-AC-66 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 | When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it ... |
| FR-024-AC-11 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:545 | `StateClauseReplay::replay` returns exactly the `StateClauseReplayResult` that `qsl_replay::replay_state_clause` returns when a test calls it directly with the same request and envelope ... |

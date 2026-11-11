---
id: SR-795004
title: "IR-1520 PR #795 spec-review/evidence"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen PR #795 at aab0b8e1 (git diff origin/main...HEAD); IR-1520; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/kani/matrix/tests.md, spec/replay/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1520. PR #795 (spec only, one commit aab0b8e1 on origin/main e75918f4, branch spec/ir-1520-postcondition-identity) implements the code lead's Option A for the postcondition/frame lane. It adds FR-015-AC-137, FR-024-AC-67 and FR-024-AC-68, a closure paragraph on the FR-015-AC-48 PARTIAL note, a pointer sentence on FR-024-AC-65, TC-025 step 54, TC-035 step 47 with its status sentence, and the matrix rows in spec/kani/matrix/tests.md and spec/replay/matrix/tests.md. Reviewed read-only against the code at origin/main (src/kani/generate/frame.rs, src/kani/identity.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/routed/capability.rs, src/replay/state_clause.rs, src/replay/frame.rs, src/replay/obligation.rs), the pinned qsl-replay at 434869e, interface-001, the open PRs #794 and #796, and the IR-1520 decision comment. No cargo or build was run. `quire validate` (make spec equivalent) on the branch reports 6 warnings, the same 6 as origin/main; `quire matrix` lists FR-015-AC-137, FR-024-AC-67 and FR-024-AC-68 as untagged, as expected for PLANNED criteria. Method: spec-review/evidence.

## Verdict

**CHANGES REQUIRED.** `Test (TC-025)` / `Test (TC-035)` is the right method for all three criteria. Step 47's contract-half mutants each name a failing case and the refusal site (identity comparison in `StateClauseReplayInputs::decode_falsification`): ignore-caller, node+model only, map-to-HarnessMismatch, harness-self, default identity and fields-first each fail a stated case. But the frame half of step 47 cannot run, and step 54 has a weak record assertion and misattributes one mutant.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `StateClauseReplayInputs::decode_falsification` accepts only a `StateFrameProperty::Postcondition` harness: its first harness check is `let StateFrameProperty::Postcondition {..} = &harness.property else { return Err(NotPostconditionHarness) }` (src/replay/state_clause.rs:474-483 at origin/main), and interface-001:264 names its input 'the falsified postcondition harness's StateFrameIdentity'. A `frame`-role harness carries `StateFrameProperty::Frame` and is replayed by a different path, `FrameReplay::new` (src/replay/frame.rs:50-72, :730), with `FrameReplayError`, not `StateClauseReplayError`. AC-67 itself places the identity check after `NotPostconditionHarness`. So 'a `frame` harness given the `contract` identity ... returns `ObligationIdentityMismatch`' can never hold: that call returns `NotPostconditionHarness` whatever the identity. Step 47's 'Repeat for the `frame` harness with I_f accepted and I_c refused' therefore fails on the accept case under the correct implementation, and the 'single identity for both roles' control's I_c-on-`frame` case is unrunnable. That mutant is a generator mutant; it is killed only by the generation assertion ('each harness's `obligation` equals its role's identity'), so name its site (`generate_state_frame_role`) and that failing assertion instead of the replay cases. | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:409-429, src/replay/state_clause.rs:474-483 |
| FND-002 | low | 'its record serialises it' is not a round trip. A mutant that writes the role's identity to the record without its ordinal (or model) keeps the in-memory equality, and the swapped regeneration still gives different records because I_c and I_f also differ in node. Add: `StateFrameIdentity::from_record(record)` returns `obligation` equal to I_c (resp. I_f) member for member. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:945-953 |
| FND-003 | low | 'one that omits the member from the record fails the parse assertion': the parse assertion parses a hand-edited record with the member removed, which a generator mutant does not touch. That mutant fails 'its record serialises it' (or the round trip of FND-002) instead. None of the four mutants names its site (`generate_state_frame_role` / the record writer), unlike step 47. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:945-953 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| TC-025-step-54 | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:945-953 | 54. (PLANNED, IR-1520.) State-frame identity member, FR-015-AC-137. Build one `StateFrameRequest` for a postcondition clause with `obligation_identities` I_c (contract) and I_f (frame) that differ in node, model and ordinal, taken from hand-written literals. Generate both roles: each harness's `obligation` equals its role's identity member for member, and its record serialises it. Regenerate with I_c and I_f swapped: the harness sources are byte-identical and the records differ. Parse a record with the member removed: `StateFrameRecordError`. Mutants: a generator that stores I_c on b |
| TC-035-step-47 | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:409-429 | 47. (PLANNED, IR-1520.) Generate, for one postcondition clause, the `contract` and `frame` harnesses of one `StateFrameRequest` whose `obligation_identities` hold two identities I_c and I_f that differ in ordinal and in node. Assert each harness's `obligation` equals its role's identity and a persisted record without the member is refused with `StateFrameRecordError` (FR-015-AC-137). Replay the falsified `contract` harness with I_c: `decode_falsification` accepts and the packet and request carry I_c member for member (FR-024-AC-68). Then supply I_c with only the ordinal changed, I_c  |
| TC-035-status-paragraph | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:471 | Step 47 (FR-024-AC-67 and FR-024-AC-68, FR-015-AC-137) is planned for IR-1520 and claims no CG implementation or tagged test yet. Step 46 (FR-024-AC-65 and FR-024-AC-66, IR-1519) is implemented by the identity comparison in `InvariantReplayInputs::decode_falsification` and covered by the tagged tests of `tests/it/state_clause_invariant_replay.rs`. Steps 39 to 45 (FR-024-AC-57 to FR-024-AC-64, IR-1505) are implemented by `StateClauseReplay::new_invariant` and covered by the tagged tests of `tests/it/state_clause_invariant_replay.rs`. Step 33 and result 38 are planned for IR-1255; the  |
| FR-015-AC-137 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:949 | PLANNED (IR-1520, the same ticket carries the code; spec IR-1520; refs FR-015-AC-48, FR-024-AC-67 and FR-024-AC-68). When `generate_state_frame_obligations` or `generate_state_frame_role` generates a harness, the harness's `StateFrameIdentity.obligation` equals member for member the QSL `ObligationIdentity` the request supplies for that role, unchanged, with no preimage, hash or derived node or ordinal. This is a public product change, explicit for review: `StateFrameRequest` gains a required member `obligation_identities` holding one identity for the `contract` role and one for the  |
| FR-024-AC-67 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:597 | PLANNED (IR-1520, the same ticket carries the code). Prerequisite: FR-015-AC-137 (the `StateFrameIdentity` member `obligation`, a new record member and a new `StateFrameRequest` member) and FR-024-AC-65. When `StateClauseReplayInputs::decode_falsification` receives a state-frame harness whose `obligation` (QSL's `ObligationIdentity`) differs in any member from the `obligation_identity` the caller put in the inputs, it returns `StateClauseReplayError::ObligationIdentityMismatch`, with the playback left unbound, no document built and the executor uncalled. The check runs after the harn |
| FR-024-AC-68 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:598 | PLANNED (IR-1520, the same ticket carries the code). Prerequisite: FR-024-AC-67. When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it, and the packet and request that `StateClauseReplay::new` builds carry the caller's identity member for member. The two roles of one request carry distinct identities, so a `contract` (postcondition) harness given the identity of the same clause's `frame` harness, and a `frame` harness given the `contract` identity, each return `ObligationIdentityMismatch`. |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head 8ad3a920: verified in the diff aab0b8e1..8ad3a920 and against origin/main code. |
| FND-002 | fixed | Round 1, head 8ad3a920: verified in the diff aab0b8e1..8ad3a920 and against origin/main code. |
| FND-003 | fixed | Round 1, head 8ad3a920: verified in the diff aab0b8e1..8ad3a920 and against origin/main code. |

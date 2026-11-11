---
id: SR-795002
title: "IR-1520 PR #795 spec-review/ears"
type: SpecReview
analysis: ears-conformance
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

Ticket: IR-1520. PR #795 (spec only, one commit aab0b8e1 on origin/main e75918f4, branch spec/ir-1520-postcondition-identity) implements the code lead's Option A for the postcondition/frame lane. It adds FR-015-AC-137, FR-024-AC-67 and FR-024-AC-68, a closure paragraph on the FR-015-AC-48 PARTIAL note, a pointer sentence on FR-024-AC-65, TC-025 step 54, TC-035 step 47 with its status sentence, and the matrix rows in spec/kani/matrix/tests.md and spec/replay/matrix/tests.md. Reviewed read-only against the code at origin/main (src/kani/generate/frame.rs, src/kani/identity.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/routed/capability.rs, src/replay/state_clause.rs, src/replay/frame.rs, src/replay/obligation.rs), the pinned qsl-replay at 434869e, interface-001, the open PRs #794 and #796, and the IR-1520 decision comment. No cargo or build was run. `quire validate` (make spec equivalent) on the branch reports 6 warnings, the same 6 as origin/main; `quire matrix` lists FR-015-AC-137, FR-024-AC-67 and FR-024-AC-68 as untagged, as expected for PLANNED criteria. Method: spec-review/ears.

## Verdict

**Minor.** The new criteria are direct assertions; `quire validate` reports no new grammar warning (6 on branch, the same 6 as origin/main). AC-67 is single-obligation (one refusal with its order). AC-137 and AC-68 are compound.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-137 states: (1) the harness's `obligation` equals the role's supplied identity; (2) `StateFrameRequest` gains the required member; (3) the persisted record gains the member and a record without it is refused with `StateFrameRecordError`; (4) the member is no input to harness source text (byte-identical source, changed record). Each has its own case and mutant in TC-025 step 54, so a test cannot be tagged to one without the others. Split into carry (1, 2), record (3) and source-independence (4) criteria. | spec/kani/functional/FR-015-bounded-kani-obligations.md:949 |
| FND-002 | medium | AC-68 asserts acceptance on equality, that the packet and request carry the caller's identity, and a role cross-refusal that is itself a refusal outcome (`ObligationIdentityMismatch`), which belongs with AC-67. Move the role cross-refusal into AC-67 (or its own criterion) and keep AC-68 to acceptance and carry, as FR-024-AC-66 does for the invariant path. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:598 |
| FND-003 | low | Both FRs keep one EARS 'The generator shall ...' bullet per obligation family (FR-015 lines 503-513 cover V2 clause claims only; FR-024 lines 449-458 list `ObligationIdentityMismatch` for the invariant path only). The PR adds criteria but no statement for the state-frame identity carry or for the postcondition path's `ObligationIdentityMismatch`, so the ACs verify a requirement the FR does not state. Add one ubiquitous bullet to FR-015 and one unwanted-behaviour bullet to FR-024. | spec/kani/functional/FR-015-bounded-kani-obligations.md:503-513, spec/replay/functional/FR-024-counterexample-envelope-intake.md:449-458 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-AC-137 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:949 | PLANNED (IR-1520, the same ticket carries the code; spec IR-1520; refs FR-015-AC-48, FR-024-AC-67 and FR-024-AC-68). When `generate_state_frame_obligations` or `generate_state_frame_role` generates a harness, the harness's `StateFrameIdentity.obligation` equals member for member the QSL `ObligationIdentity` the request supplies for that role, unchanged, with no preimage, hash or derived node or ordinal. This is a public product change, explicit for review: `StateFrameRequest` gains a required member `obligation_identities` holding one identity for the `contract` role and one for the  |
| FR-024-AC-67 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:597 | PLANNED (IR-1520, the same ticket carries the code). Prerequisite: FR-015-AC-137 (the `StateFrameIdentity` member `obligation`, a new record member and a new `StateFrameRequest` member) and FR-024-AC-65. When `StateClauseReplayInputs::decode_falsification` receives a state-frame harness whose `obligation` (QSL's `ObligationIdentity`) differs in any member from the `obligation_identity` the caller put in the inputs, it returns `StateClauseReplayError::ObligationIdentityMismatch`, with the playback left unbound, no document built and the executor uncalled. The check runs after the harn |
| FR-024-AC-68 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:598 | PLANNED (IR-1520, the same ticket carries the code). Prerequisite: FR-024-AC-67. When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it, and the packet and request that `StateClauseReplay::new` builds carry the caller's identity member for member. The two roles of one request carry distinct identities, so a `contract` (postcondition) harness given the identity of the same clause's `frame` harness, and a `frame` harness given the `contract` identity, each return `ObligationIdentityMismatch`. |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | Delivered (IR-1519). Prerequisite: FR-024-AC-62 and the `KaniV2ClauseObligationHarness.obligation` member of FR-015-AC-48 (IR-1507, merged). When `InvariantReplayInputs::decode_falsification` receives a V2 invariant harness whose `obligation` (QSL's `ObligationIdentity`) differs in any member from the `obligation_identity` the caller put in the inputs, it returns the distinct typed `StateClauseReplayError::ObligationIdentityMismatch`, with the playback left unbound, no document built and the executor uncalled. The check runs after the harness-kind check (`NotInvariantHarness`) and be |
| FR-015-AC-48 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 | A V2 clause claim carries QSL's producer-issued structural `ObligationIdentity` unchanged: the obligation node's `NodeKey`, optional owning `ModelIdentity`, and producer-assigned ordinal for that node/model pair (AD-003 E-1). Distinct obligations at the same node/model have distinct ordinals; changing any structural member changes the identity. CG does not form a preimage, hash the identity, or derive a replacement node or ordinal. CG compares the complete source occurrence, obligation kind, parameter identities and declared domains, and any state-field read identities and domains, a |
| TC-025-step-54 | context_only | spec/kani/matrix/TC-025-bounded-kani-obligations.md:945-953 | 54. (PLANNED, IR-1520.) State-frame identity member, FR-015-AC-137. Build one `StateFrameRequest` for a postcondition clause with `obligation_identities` I_c (contract) and I_f (frame) that differ in node, model and ordinal, taken from hand-written literals. Generate both roles: each harness's `obligation` equals its role's identity member for member, and its record serialises it. Regenerate with I_c and I_f swapped: the harness sources are byte-identical and the records differ. Parse a record with the member removed: `StateFrameRecordError`. Mutants: a generator that stores I_c on b |
| TC-035-step-47 | context_only | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:409-429 | 47. (PLANNED, IR-1520.) Generate, for one postcondition clause, the `contract` and `frame` harnesses of one `StateFrameRequest` whose `obligation_identities` hold two identities I_c and I_f that differ in ordinal and in node. Assert each harness's `obligation` equals its role's identity and a persisted record without the member is refused with `StateFrameRecordError` (FR-015-AC-137). Replay the falsified `contract` harness with I_c: `decode_falsification` accepts and the packet and request carry I_c member for member (FR-024-AC-68). Then supply I_c with only the ordinal changed, I_c  |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head 8ad3a920: verified in the diff aab0b8e1..8ad3a920 and against origin/main code. |
| FND-002 | fixed | Round 1, head 8ad3a920: verified in the diff aab0b8e1..8ad3a920 and against origin/main code. |
| FND-003 | fixed | Round 1, head 8ad3a920: verified in the diff aab0b8e1..8ad3a920 and against origin/main code. |

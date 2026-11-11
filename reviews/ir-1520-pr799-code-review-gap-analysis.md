---
id: SR-17940486709564847047702
title: "IR-1520 PR #799 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #799 at 0945ae7a; IR-1520; FR-015-AC-137 to FR-015-AC-142, FR-024-AC-71 to FR-024-AC-75, TC-025 step 54, TC-035 step 47; tests/it/state_frame_obligation_identity.rs, src/kani/generate/role_parity_tests.rs, tests/common/state_frame_identities.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-1520. Planless gap analysis of the eleven criteria the PR flips to Implemented. Plan
completion: not assessed.

`quire matrix --scope .` on the PR head lists all eleven criteria as `tagged`:
FR-015-AC-137 (:69), AC-138 (:97, :162), AC-139 (:193), AC-140 (:262), AC-141 (:336), AC-142 (:391),
FR-024-AC-71 and AC-72 (:446), AC-73 and AC-74 (:524), AC-75 (:524, :624), all in
`tests/it/state_frame_obligation_identity.rs`. Each binding was read against its criterion; each
test asserts its criterion's outcome on real generated harnesses (an authentic QSL twin), not on a
stub. Expected identities are hand-written literals (`tests/common/state_frame_identities.rs`) or
identities QSL's own selected sites mint (`OperationSite::obligation_identity`,
`Twin::clause_identity`), never computed by CG code under test. No production code without an owning
requirement: the only addition beyond the merged spec, the writer depth bound, is now owned by the
edited FR-015-AC-138 row (its spec standing is judged in the code-review and spec-review artifacts).

Mutation evidence I ran myself (see the code-review artifact for the run): MA (postcondition
compare ignoring `model`), MB (frame compare ignoring `node`), MC (postcondition identity check
moved before the kind check) and MD (frame mismatch returned as `NotAFrame`) are each killed by the
named test; ME (depth bound 32 -> 40) survives.

## Verdict

**Coverage complete for all eleven criteria; two low evidence-precision gaps.** No untagged
criterion, no stub, no tautological oracle.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-015-AC-137's evidence says `role_parity_tests::state_frame_both_roles_match_direct_generation` "asserts the identity of `generate_state_frame_role` for each role", but it only asserts the negotiated harness equals the direct `generate_state_frame_role` output, both built with the same identity; it never compares `obligation` to the passed identity. The public test uses `generate_state_frame_obligations`, which does not call `generate_state_frame_role` (it calls `frame_harness`/`contract_harness`). The carry through `generate_state_frame_role` is in fact pinned by the negotiate test (AC-141), so TC-025 step 54(a)'s `generate_state_frame_role` case is covered only indirectly and the evidence sentence misattributes it | src/kani/generate/role_parity_tests.rs:232-246, tests/it/state_frame_obligation_identity.rs:69-91, spec/kani/functional/FR-015-bounded-kani-obligations.md:952 |
| FND-002 | low | FR-024-AC-72 requires the accepted identity to have "an `ordinal` and a `node` that differ from every other identity the test constructs"; the test's accepted `ids::contract()` (node ids/contract/7, ordinal 3) shares its node with `contract_ordinal_only` and `contract_model_only` and its ordinal with `contract_node_only` and `contract_model_only` | tests/common/state_frame_identities.rs:13-34, tests/it/state_frame_obligation_identity.rs:459-488 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-137 | examined | generate_state_frame_role returns a harness whose StateFrameIdentity.obligation equals, member for member, the QSL ObligationIdentity passed for that role; generate_state_frame_obligations returns the contract harness carrying the contract identity and the frame harness carrying the frame identity. |
| FR-015-AC-138 | examined | The persisted record writes obligation as node (package, path with {"text"} and {"node"} segments, ordinal), optional model and ordinal; from_record rebuilds it equal member for member. |
| FR-015-AC-139 | examined | from_record refuses a record lacking obligation, or whose obligation lacks node or ordinal, with StateFrameRecordError. |
| FR-015-AC-140 | examined | Two generations differing only in the identity give byte-identical harness source and records differing only in obligation. |
| FR-015-AC-141 | examined | Each StateFrame item's harness carries its own identity; equal identities across items are each carried and not refused. |
| FR-015-AC-142 | examined | StateFrameRoleIdentities::new refuses a contract identity equal in every member to the frame identity with SharedIdentity. |
| FR-024-AC-71 | examined | A postcondition harness whose obligation differs from the caller's returns ObligationIdentityMismatch after NotPostconditionHarness and before HarnessMismatch. |
| FR-024-AC-72 | examined | An equal identity is accepted and carried member for member into packet and request, with an ordinal and a node that differ from every other identity the test constructs. |
| FR-024-AC-73 | examined | FrameReplay::new compares the minted frame identity with the harness obligation and refuses a difference with FrameReplayError::ObligationIdentityMismatch before the request is built. |
| FR-024-AC-74 | examined | A frame harness carrying the minted identity is accepted and the request and packet carry it. |
| FR-024-AC-75 | examined | FrameReplayError::ObligationIdentityMismatch settles as ReplaySettlement::CgDefect. |

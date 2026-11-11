---
id: SR-17940486709564847047693
title: "IR-1519 PR #793 obligation-identity cross-check: gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen PR #793 at 48e18b45; IR-1519; FR-024-AC-65, FR-024-AC-66, TC-035 step 46; src/replay/state_clause/invariant.rs, tests/it/state_clause_invariant_replay.rs, tests/common/v2_claim.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1519. This is a planless gap analysis (Plan completion: not assessed).

`quire matrix --scope .` binds the two criteria to one tagged test each:

| Criterion | Test | Location |
| --- | --- | --- |
| FR-024-AC-65 | `tc_035_a_different_obligation_identity_is_refused_before_the_harness_members` | tests/it/state_clause_invariant_replay.rs:1107 |
| FR-024-AC-66 | `tc_035_an_equal_obligation_identity_is_accepted_and_carried_unchanged` | tests/it/state_clause_invariant_replay.rs:1044 |

Both are listed as tagged, and both tests pass.

Every TC-035 step 46 assertion is present:

- Accept, then carry member for member into `packet.obligation_identity` and `wire.obligation_identity`, with ordinal 7, a model, and a node unlike the decoy's.
- Ordinal-only, model-only, node-only and all-member differences each give `ObligationIdentityMismatch`, never `HarnessMismatch`, with the playback unbound and a `CgDefect` settlement.
- Identity plus field order gives the identity refusal.
- The same field order with a matching identity gives `HarnessMismatch(StateFields)`.

Each listed mutation control is killed. I ran the default-identity mutant and the after-members mutant myself; the coder ran the other four. See the code-review SR for the full mutant table.

## The fixture change to caller identity

The PR changes six existing invariant tests (tests/it/state_clause_invariant_replay.rs:133, 593, 638, 660, 845, 902). Each now sets `inputs.obligation_identity = <harness>.obligation.clone()` instead of the default from `Twin::invariant_inputs`. That default is QSL's call-site identity: the `clause_site` node, model None, and the occurrence ordinal.

I removed the line at :133 to probe the change. Eight of the 10 tests then refuse with `ObligationIdentityMismatch`. A diagnostic print shows that model and ordinal agree and only the node differs:

- QSL's resolved `NodeKey` is structural: `["state","state_clause","invariant",node(...Account...),node(...binary...)]`.
- The test helper `tests/common/v2_claim.rs::node_key` produces `["state","state_clause","invariant","Index(0)","Index(3)"]`. It renders `CheckedNodePathSegment::Index` through the `format!("{other:?}")` fallback.

So the fixture change does not hide a CG production defect. The fixture claims carry an arbitrary NodeKey, and FR-015-AC-48 says the NodeKey is not checked until FR-015-AC-135 and AC-136 land under IR-1518. IR-1518 owns the fixture fallout ("existing V2 test fixtures carry arbitrary NodeKeys and need real keys").

The cost is that after this PR no test shows that a caller holding QSL's own call-site identity is accepted by a CG-generated invariant harness. Before the PR, the cross-check did not exist, so nothing compared the two. `Twin::invariant_inputs` still builds the call-site identity as its default, and every invariant harness now refuses it. That is FND-001.

Is there still a test where the call-site identity legitimately differs from the harness claim? No. The only differing identities left are the hand-built AC-65 decoys, and those are refusal cases.

## Verdict

**MERGEABLE.** FR-024-AC-65 and FR-024-AC-66 are implemented and covered as specified, with no stub, tautology or coverage inflation.

FND-001 is a medium coverage gap. It is caused by test claims carrying arbitrary keys, a problem IR-1518 already owns. Defer it to IR-1518 rather than fix it here: a real resolved key in `claims_for` is IR-1518's change. Once that lands, restore the call-site identity at the six sites, or assert it equals `harness.obligation` once in `Fixture::decoded`. The cross-check is then exercised end to end against QSL's identity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The six existing invariant tests now copy `harness.obligation` into the caller's `obligation_identity`. As a result, no test checks that QSL's call-site identity, which a real caller holds and which `Twin::invariant_inputs` still builds, matches a CG invariant harness's claim. I probed this by dropping the copy at :133: 8 of 10 tests refuse with `ObligationIdentityMismatch`. Only the node differs, because the test helper `v2_claim::node_key` renders Index path segments as Debug text (`"Index(0)","Index(3)"`) instead of QSL's structural key. This is not a production defect: the claim NodeKey is unchecked until FR-015-AC-135 (IR-1518). Defer to IR-1518, which should restore the call-site identity, or an equality assertion against it, in `Fixture::decoded` and the other five sites once the claims carry resolved keys. | tests/it/state_clause_invariant_replay.rs:133; tests/common/v2_claim.rs:12-23; tests/state_frame_support/native_twin.rs:554-561 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | When `InvariantReplayInputs::decode_falsification` receives a V2 invariant harness whose `obligation` differs in any member from the `obligation_identity` the caller put in the inputs, it returns the distinct typed `StateClauseReplayError::ObligationIdentityMismatch` |
| FR-024-AC-66 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 | When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it, and the packet and the request carry the caller's identity member for member, with an `ordinal` and a `node` that differ from every other identity the test constructs. |
| FR-024-AC-62 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:592 | a harness whose QSL `ObligationIdentity` differs from the caller's returns `ObligationIdentityMismatch` (FR-024-AC-65) |
| FR-015-AC-48 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 | Until those land, the code compares a sequence field's inclusive cardinality range only and does not check the `NodeKey`. |
| FR-015-AC-135 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:947 | A V2 clause claim whose `ObligationIdentity` `NodeKey` is not equal ... to the `NodeKey` CG resolves for the selected clause node ... is refused with `UnsupportedObligation::V2ClauseNodeKeyMismatch` |
| TC-035 step 46 | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:394-408 | Over the step 45 invariant harness, supply a caller `obligation_identity` equal to the harness's `obligation` and assert `decode_falsification` accepts it |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | deferred | Round 1, head 6a195566. Deferred to IR-1518, which owns it in comment f49a6b22. Once #794 gives V2 claims resolved NodeKeys, restore the QSL call-site identity, or assert it equals `harness.obligation`, at the six sites in tests/it/state_clause_invariant_replay.rs, after #793 merges. |

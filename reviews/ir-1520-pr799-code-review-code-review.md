---
id: SR-17940486709564847047701
title: "IR-1520 PR #799 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #799 at 0945ae7a (base main 354e7c6f); IR-1520; src/kani/identity.rs, src/kani/identity/obligation_wire.rs, src/kani/generate/frame.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/kani/generate/role_parity_tests.rs, src/kani/test_support.rs, src/lib.rs, src/replay/frame.rs, src/replay/state_clause.rs, tests/common/state_frame_identities.rs, tests/it/state_frame_obligation_identity.rs, tests/state_frame_support/native_twin.rs, fallout edits in tests/it"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
---

## Summary

Ticket: IR-1520. The PR carries one QSL `ObligationIdentity` per role on the state-frame lane and
cross-checks it on replay. Every coder fact in the brief was checked against the code:

- `StateFrameIdentity` gains the required public member `obligation`, serialised by the new
  `obligation_wire` module as `{"node": {"package", "path", "ordinal"}, "model": {"identity"}?,
  "ordinal"}`; a `Text` segment is `{"text": s}` and a `Node` segment `{"node": <node>}`, nested
  recursively (`Segments`/`Node` serialisers, `SegmentWire::Node(NodeWire)` reader). The reader is
  `deny_unknown_fields`, has no `#[serde(default)]` on `node`, `ordinal`, `package`, `path` or the
  node `ordinal`, and rebuilds through `NodeKey::new` and `ObligationIdentity::new`. A missing member
  surfaces as `StateFrameRecordError::Json`.
- `StateFrameRoleIdentities::new` returns `SharedIdentity` for a pair equal under the derived
  `PartialEq` of `ObligationIdentity` (all three members).
- `generate_state_frame_role(request, role, &ObligationIdentity)`;
  `generate_state_frame_obligations(request, StateFrameRoleIdentities)` takes the pair by value and
  destructures it into the `contract` and `frame` harness builders; `StateFrameRequest` is unchanged.
- `ObligationItem::StateFrame` has the required `obligation: &'a ObligationIdentity`, threaded to
  `classify_state_frame` -> `generate_state_frame_role`. Nothing compares identities across items.
- `classify_state_frame_role` (routed preflight, `src/routed/capability.rs:939`) takes no identity.
  Note: negotiate's disposition does not call it; it calls `generate_state_frame_role` (see FND-001).
- `StateClauseReplayInputs::decode_falsification`: the identity comparison sits directly after the
  `NotPostconditionHarness` let-else and before `ClauseShape::read` and the `HarnessMismatch` loop.
- `FrameReplay::new`: `frame_identity(&site, kind)` is minted after the anchor and frame
  `same_node` checks, compared with `harness.obligation`, and a difference returns the new
  `FrameReplayError::ObligationIdentityMismatch` before `SelectedFrameClaim`, the witness, the
  request and the packet are built. The `From<&FrameReplayError> for ReplaySettlement` arm maps it to
  `CgDefect`.

Panic surface: no new `unwrap`/`expect`/`panic!`/indexing/`as` casts in non-test `src`. No hashes,
digests, git pins or lock changes. Every new test carries `#[trace(..)]` (ix-trace-rs). Breaking
public API matches FR-015-AC-137/AC-141/FR-024-AC-73 and is listed completely in the PR body
(`generate_state_frame_obligations` argument, `ObligationItem::StateFrame.obligation`,
`StateFrameIdentity.obligation`, two new public types, new `FrameReplayError` variant;
`FrameReplayError` is not `#[non_exhaustive]`, so the variant is breaking for exhaustive matchers,
as recorded). No schema, README, docs or doctest names the changed signatures.

The writer depth bound (`MAX_NODE_DEPTH = 32`) was judged. The arithmetic is right: the record nests
`RecordView` > `identity` > `obligation` > node, and each node level adds three containers (node
object, `path` array, segment object), so a node at depth d puts its innermost segment at container
3d+3 and serde_json's 128-container read limit is first exceeded at about d = 42; 32 leaves
about nine levels of margin, which the doc comment does not derive. It is a deviation from the merged
spec: FR-015-AC-137 and AC-141 say a harness is returned carrying any passed identity, and the bound
turns a deep identity into `StateFrameRefusal::RecordSerialization` (negotiate:
`Unsupported(RenderFailed)`). Refusing is better than writing an unreadable record (AC-138's round trip
could not hold), so the behaviour is defensible, but it belongs in the spec text of AC-137, AC-141 and
interface-001, not only as a sentence added to the AC-138 row (FND-001, and spec-review FND-002). The
boundary itself is not tested (FND-002).

M9 (classification reading the identity) is correctly called inexpressible for
`classify_state_frame_role`, which has no identity parameter. The spec's premise that negotiate's
disposition goes through that function is false, and on negotiate's real path an identity-dependent
disposition is expressible and occurs (FND-001).

Executed in my own worktree (`flock /tmp/cargo-slot.lock`, `CARGO_TARGET_DIR` in the worktree,
`QUIRE_CVC5_BIN` set), `cargo test --locked --test it` with separate filters:

- baseline: `state_frame_obligation_identity::` 10/10, `layout::` 8/8, `no_generation_panics::` 7/7
  (including `tc_042_ac1_src_holds_no_panic_token`), `kani_gate::` 11/11, `frame_claim` 1 passed and
  2 ignored (QSL-1336, pre-existing). These are the three gates the coder did not run (layout,
  no_generation_panics, kani_gate); all pass.
- My own mutants (each applied, run, reverted; `git status` clean afterwards), filter
  `state_frame_obligation_identity::`:
  - MA, `decode_falsification` comparing only `node` and `ordinal` (ignoring `model`): killed,
    `tc_035_postcondition_decode_cross_checks_the_identity` (:475, the model-only case accepted).
  - MC, the `decode_falsification` identity check moved before the `NotPostconditionHarness`
    kind check: killed, the same test (:513, a frame harness now returns ObligationIdentityMismatch).
  - MB, `FrameReplay::new` comparing only `model` and `ordinal` (ignoring `node`): killed,
    `tc_035_frame_replay_cross_checks_the_minted_identity` (:578, the node-only case accepted).
  - MD, `FrameReplay::new` returning `NotAFrame` instead of `ObligationIdentityMismatch`: killed,
    `tc_035_frame_replay_cross_checks_the_minted_identity` (:581) and
    `tc_035_the_frame_identity_mismatch_settles_as_a_cg_defect` (:632).
  - ME, `MAX_NODE_DEPTH` 32 -> 40: survives, 10/10 pass (FND-002).

No Kani, no `make ci`, no full suite.

## Verdict

**Not mergeable as is; code correct on every specified path.** The cross-checks are placed and
ordered as FR-024-AC-71 and AC-73 require, the wire form is exactly AC-138's, and the tests are
hand-written literals plus QSL-minted identities, not values derived from the code under test. My
mutants on both replay checks are killed. The medium finding is a spec/code agreement gap introduced
by the unspecified writer depth bound; it needs spec text (or a decision to keep the bound documented
only in AC-138) in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The unspecified writer depth bound makes generation and negotiate's disposition depend on the identity: an identity nested past 32 levels yields `StateFrameRefusal::RecordSerialization` (negotiate `Unsupported(RenderFailed)`) while the routed preflight `classify_state_frame_role` still reports the item supported; FR-015-AC-137 and AC-141 as written promise a harness carrying any passed identity and a result that is the same for any identity | src/kani/identity/obligation_wire.rs:21, src/kani/identity/obligation_wire.rs:124-128, src/kani/generate/frame.rs:1150-1154, src/kani/generate/negotiate.rs:1148-1156, src/routed/capability.rs:939 |
| FND-002 | low | The depth bound's boundary is not tested: the only cases are depth 9 (accepted) and depth 201 (refused), so moving `MAX_NODE_DEPTH` (mutant ME, 32 -> 40) survives, and the reader-side depth refusal in `node_from_wire` has no test at all | tests/it/state_frame_obligation_identity.rs:162-188, src/kani/identity/obligation_wire.rs:53-56 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-137 | examined | generate_state_frame_role returns a harness whose StateFrameIdentity.obligation equals, member for member, the QSL ObligationIdentity passed for that role, unchanged. |
| FR-015-AC-138 | examined | The persisted state-frame record writes obligation as an object holding node, model and ordinal; a Node segment is {"node": <node object>}; from_record rebuilds it equal. |
| FR-015-AC-139 | examined | from_record refuses a record that lacks obligation, or whose obligation lacks node or ordinal, with StateFrameRecordError. |
| FR-015-AC-140 | examined | The identity is not an input to harness source text, the harness symbol or the module symbol. |
| FR-015-AC-141 | examined | ObligationItem::StateFrame gains a required obligation member; negotiation compares nothing across items; classification does not read the identity. |
| FR-015-AC-142 | examined | StateFrameRoleIdentities::new refuses an equal pair with SharedIdentity. |
| FR-024-AC-71 | examined | decode_falsification returns ObligationIdentityMismatch after NotPostconditionHarness and before the HarnessMismatch checks. |
| FR-024-AC-72 | examined | An equal identity is accepted and carried into packet and request. |
| FR-024-AC-73 | examined | FrameReplay::new compares the minted identity with the harness's obligation before the request is built; new variant ObligationIdentityMismatch. |
| FR-024-AC-74 | examined | An equal minted identity is accepted and carried into request and packet. |
| FR-024-AC-75 | examined | FrameReplayError::ObligationIdentityMismatch settles as ReplaySettlement::CgDefect. |
| src/replay/obligation.rs frame_identity | context_only | function_contract_identity(site.obligation_identity(), &site.frame_occurrence, kind, &[]) |

---
id: SR-794001
title: "IR-1518 PR #794 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #794 at 1da7a721; IR-1518; src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/kani/test_support.rs, tests/common/v2_claim.rs, tests/it/v2_clause_obligation_identity.rs, tests/it/kani_collection_operation.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-1518. `classify_v2_clause` gains a guard arm that refuses a V2 clause claim whose
`ObligationIdentity` `NodeKey` differs from `package.structural_key(clause).and_then(checked_node_key)`
(full `NodeKey` equality; a resolver `None` compares unequal to `Some(claim key)`) with the new
`UnsupportedObligation::V2ClauseNodeKeyMismatch { node_id }`. The arm sits after the UnknownNode,
UnknownNodeKind and unsupported-kind refusals and before `lower_v2_clause` /
`lower_collection_operation`, so it precedes every record comparison (which runs on the lowered
selection) on both the scalar and the whole-object collection route. `classify_v2_clause` is the
only caller of either lowerer. The resolver is the same expression the replay seam uses
(`src/replay/state_clause.rs:617-623`). No request member was added. The rust-review lane is folded
into this file.

Refusal surface: the variant carries only `node_id: CheckedNodeId`; it derives `Serialize` under the
enum's `#[serde(tag = "code", rename_all = "snake_case")]`, so its wire code is the stable
`v2_clause_node_key_mismatch`, and the variant doc comment states the contract (FR-015-AC-135/136,
names the clause node only). `UnsupportedObligation` is `#[non_exhaustive]`; no exhaustive match on it
exists in `src/` outside its constructors, and quire-driver (`tests/drive.rs`) matches only specific
variants, so nothing must be mapped in this PR (driver adaptation is IR-1494/1504/1511).

Test custody: the new 53(a)-(d) tests compare against `clause_key(...)`, a key literal assembled by
hand from package names, path text segments and ordinals; it calls no resolver, so a mutated
resolver cannot move both sides. The fallout helpers (`resolved_key` in `tests/common/v2_claim.rs`,
`clause_obligation` in `src/kani/test_support.rs`) make the claim key equal the resolved key only in
fixtures that do not exercise the binding; the differing-key cases (53(b) another clause's key,
53(c) ordinal 1, 53(d) another key plus a widened record) remain and are independent of them.

Executed (own worktree, `flock /tmp/cargo-slot.lock`, `QUIRE_CVC5_BIN` set): focused job
`cargo test --lib --test it -- v2_clause_obligation_identity kani_collection_operation generate::negotiate`:
lib 7/7, it 28/28 pass. Mutation controls (each reverted, `git status` clean afterwards):
M5 (resolve the key of the first classified clause, B, instead of the selected one) fails 53(a),
53(b) and 53(d) and survives 53(c), as predicted (spec predicts a and b; d also fails because B's key
then passes for A and the record comparison runs). M7 (compare package and path, not the node
ordinal) fails 53(c) only. Coder's M4/M6/M8 results were not re-run. No Kani, no full suite.

No `src/replay/state_clause*` file is touched. `git merge-tree` against #792 (IR-1514) and #793
(IR-1519) is clean; #793's spec edits are in `spec/replay/**`, disjoint from this PR's `spec/kani/**`.

## Verdict

**PASS (mergeable).** Implementation matches FR-015-AC-135/136; the tests are not tautological and kill
M5 and M7. One low readability nit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new guard's `kind.is_some() &&` is redundant (the preceding arm already refuses every kind other than Some(Precondition/Postcondition/Invariant)), and the following arm's `None => V2ClauseConditionNotRendered` is unreachable; the dead conjunct reads as if a `None` kind could skip the key check | src/kani/generate/negotiate.rs:952-981 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| classify_v2_clause | examined | Refuses a claim whose NodeKey is not the resolved key of the selected clause node, before lower_v2_clause and the record comparison. |
| UnsupportedObligation::V2ClauseNodeKeyMismatch | examined | The claim's obligation NodeKey is not the key CG resolves for the selected clause node; names the clause node only. |
| tests/it/v2_clause_obligation_identity.rs 53(a)-(d) | examined | Two clauses A and B, B listed first; hand-written keys; admitted control, swapped key, ordinal 1, key plus record mismatch. |
| resolved_key / clause_obligation / claim_of / hand_written_claim | examined | Fixture claims carry the resolved key of the clause they claim. |
| Twin::with_preconditions_on_two_operations | examined | Two preconditions on deposit and transfer of Account reading balance and audit. |
| src/replay/state_clause.rs:617-623 | context_only | structural_key then checked_node_key compared with the site NodeKey. |

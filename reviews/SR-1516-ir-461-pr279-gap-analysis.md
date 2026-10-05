---
id: "SR-1516"
title: "CG PR 279 gap analysis: FR-015-AC-59 to AC-65 and AC-68, the state/frame arm Behavior and mapping table, TC-025 steps 19 to 28, interface-001, AD-004 step 4d"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@c7d6517431a207415d625d35ef2d389e331b511f; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/{TC-025-bounded-kani-obligations,tests}.md, spec/core/functional/interface-001-codegen-api.md, spec/assurance/AD-004-cg-crate-layout.md, src/kani/generate/{frame,negotiate,outcome}.rs, tests/it/kani_obligations_state_frame.rs (diff origin/main...HEAD, merge base bcce798)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: references
---

# SR-1516: CG PR 279 gap analysis

## Summary

Ticket: IR-461 (code PR 2 of 2). PR: agent-ix/quire-contract-codegen#279 at c7d6517, diffed
against origin/main bcce798. This analysis is PR-scoped. Code review is SR-1515 and the
spec-review is SR-1517.

Matrix (`quire coverage --scope . --json`, quire 0.36.1, measured on both trees):

- Base bcce798: 303/442 backed, 66 unbacked rows. Head: 311/442 backed, 66 unbacked rows.
  `status_lies` and `suspicions` are empty on the head.
- The +8 are FR-015-AC-59, AC-60, AC-61, AC-62, AC-63, AC-64, AC-65 and AC-68, each now tagged
  by one test in tests/it/kani_obligations_state_frame.rs. AC-60 and AC-68 are among the
  backed rows, although their AC text and their matrix row say PLANNED (FND-001).

AC-by-AC (semantic check of intent, test and code):

- AC-59: nine items with the first refused. The test asserts one record per item, the
  request_index sequence, the per-role `kind`, each later record equal to the same item's
  record when negotiated alone, three refused items as three records, and a later invalid item
  rejecting the request with both records. Backed.
- AC-60: the harnesses come back in request order in `state_frame_harnesses`, each record
  carries its harness symbol, the harnesses equal `generate_state_frame_obligations`' for the
  OK clause, the item's own subject path is called and the request's is not. The comparison
  with `generate_state_frame_role` is not tested (FND-001).
- AC-61: the plain-integer member is `requires_bound` with T_INTEGER. The no-bound-reachable
  lowering is `requires_bound` for both roles. Its frame-role record can only come from the
  lowering, so the lowering path is proven. Backed.
- AC-62: `NoFiniteEncoding` naming the function node and its tag, and `UnknownNodeKind` for a
  frame node, are asserted exactly. Backed.
- AC-63: 19 rows, each checking the exact refusal variant and, for bounds, the exact cause.
  This closes SR-1503 FND-002 for the three reachable causes. `MemberAbsent` and the
  invalid-body, incomplete and over-budget lowering records are asserted only through the
  AC-66 mapping, and the row says so (FND-002).
- AC-64: the InvalidStatePath cases (state path and item subject path), the InvalidStateField
  cases (bad name, and a lacking field), UnknownNode, DuplicateItem `{first_index: 0}` and
  MixedStatePackages are each asserted exactly. Each engine-invalid item alone, beside a
  supported item, rejects the request. Backed.
- AC-65: EmptyRequest, TooManyItems, InvalidSubjectPath and InvalidUnwind at 0 and at max+1,
  each the existing variant. Backed.
- AC-68: the records' independence and `generate_state_frame_obligations`' refusal of the
  failing role are asserted, and a refused lowering refuses both items alike. The `supported`
  harnesses are checked only for their property kind and clause, not compared with
  `generate_state_frame_role` (FND-001).

Mapping table and Behavior: the arm calls `state_frame_disposition` (PR 1) once per refused
item, and the `allow(dead_code)` is gone. On the "malformed clause is common" reading
(ambiguity 2): FR-015's independence paragraph lists "a malformed clause" among the grounds
common to both roles, beside the request, the lowering, the node kind and a non-postcondition.
`prepare` refuses both roles when either the frame decode or the condition decode yields
`MalformedClause`, and KEYWORD_READ asserts both roles refused. The reading is right. One
limit, inherited and not a defect: a malformed node is detected only if the decode reaches
it. A condition refused as unsupported at its first operand never reads its second.

Grounds with no fixture (ambiguity 4): `MemberAbsent` is unreachable through an admitted
package (IR refuses a read of an undeclared member; PR 1's row). The three lowering records
reach the arm through the same single `state_frame_disposition` call as every other refusal,
and AC-66 pins their mapping. The residual risk is small. FND-002 records the matrix wording.

Test-oracle strength: I ran 12 hand mutants with `cargo test --test it
kani_obligations_state_frame`, restoring the source between runs.

| Mutant | Result |
| --- | --- |
| stop at the first refused record (short-circuit) | killed (7 tests) |
| swap the roles in `generate_state_frame_role` | killed (5) |
| drop `DuplicateItem` | killed (AC-64) |
| dedupe identity without the role | killed (7) |
| the arm reads the request's `subject_path` | killed (AC-60, AC-64) |
| `state_frame_harnesses` reversed | killed (AC-60, AC-68) |
| records reversed | killed (6) |
| refused item recorded `unsupported` with a generic reason (`RenderFailed`) | killed (4) |
| `MalformedClause` not common | killed (AC-63) |
| engine-invalid item does not reject (Rejected rule) | killed (AC-59, AC-64) |
| drop `MixedStatePackages` | killed (AC-64) |
| combined entry: contract role before frame role | SURVIVED, and also survives the full `cargo test --all-targets` (SR-1515 FND-001) |

The PR's three claimed killed mutants (short-circuit, the Rejected rule, the common malformed
refusal) reproduce.

TC-025: steps 19 and 21 to 27 are backed as the new paragraph says. Steps 20 and 28 are backed
apart from the `generate_state_frame_role` comparison, and the paragraph says that. Its
"twenty-three shapes" matches `arm_shapes()`. The 8 MiB ceiling applies only to the test's own
package read (`CheckedPackageReadLimits { bytes: 8 << 20, ..bounded() }`), so production
limits are unchanged. That is acceptable for a fixture that holds 23 shapes on top of the
corpus package.

interface-001 and AD-004: consistent with the code. The AD-004 amendment is judged in SR-1517.

Reverse gap: every new non-test item (`StateFrameRole`, `obligation_kind`,
`generate_state_frame_role`, `Prepared`, `prepare`, `require_state_field`, `contract_harness`,
`frame_harness`, `classify_state_frame`, the `ItemIdentity`/`Outcome` arms) is owned by the
FR-015 IR-461 paragraph. No stubs.

Overlap with open PR #278 (IR-460): `git merge-tree` of the two heads conflicts in one hunk of
tests/it/kani_obligations_state_frame.rs. #279 scopes `SELF` and `OTHER` per shape, and #278
adds a `RESULT` binder. The fix is mechanical, but whichever PR merges second must also scope
`RESULT` through `shape.scoped(..)` and rerun the gates. frame.rs, lib.rs, layout.rs, AD-004,
interface-001 and tests.md auto-merge. #278 only widens visibility in frame.rs (`Graph`,
`ClauseShape`, `field_range`), so I see no semantic clash with the role split.

Plan completion: not assessed (planless, PR-scoped run).

## Verdict

CONDITIONAL on FND-001. The arm is correct and strongly tested, but the backed count
overstates the matrix. Resolve FND-001 by one of these:

- (a) A spec-lane amendment that re-targets AC-60 and AC-68's comparison so a test can check
  it. The comparison holds by construction: the arm passes `generate_state_frame_role`'s
  output through unchanged.
- (b) An in-crate test. This repo has no V2 package builder under `src/`, so it needs one,
  without copying tests/it/package.rs.
- (c) Drop `FR-015-AC-60` and `FR-015-AC-68` from the two tests' `Trace:` lines until (a) or
  (b) lands. Strict then reads 309/442 (FR-015 45/68), which is the truthful count today.

Leaving the status PLANNED while keeping the tags is not acceptable as it stands.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-60 and AC-68 say "PLANNED (IR-461)" in their own text, and their tests.md row says "🚧 Planned … partly tested". But `tc_025_a_supported_state_frame_item_returns_the_harness_the_engine_generates` and `tc_025_the_two_roles_of_a_state_clause_settle_independently` carry `Trace: FR-015-AC-60` and `Trace: FR-015-AC-68`, so `quire coverage` counts both as backed: 311/442, and FR-015 47/68 in the PR's own claim. quire's vocabulary does not flag Planned-but-tagged (`status_lies` is empty), so nothing shows the inconsistency. The strict count is 2 higher than what the matrix declares. The untested clause, the comparison with `generate_state_frame_role` for the split-role clause, is exactly what AC-68 adds over AC-59 and AC-63. Make the count and the status agree: either close the comparison (a spec-lane re-wording, or an in-crate test) and flip both to Covered, or untag both until then | tests/it/kani_obligations_state_frame.rs:1356, tests/it/kani_obligations_state_frame.rs:1783, spec/kani/matrix/tests.md:25 |
| FND-002 | low | AC-63 is marked ✅ Covered, but four of the cases it lists (`MemberAbsent`, and lowering invalid-body, incomplete and over-budget) are never driven through the `StateFrame` arm. They are asserted only by the AC-66 mapping test, and the row discloses this. The matrix applies two standards: AC-63 is Covered with untested clauses, while AC-60 and AC-68 are Planned with untested clauses. Either reach the over-budget lowering through the arm (a closure past `LOWERING_WORK_LIMIT` should be buildable) or state in the AC-63 row why the clauses count as covered: the arm passes every refusal through one call that AC-66 pins | spec/kani/matrix/tests.md:24, src/kani/generate/negotiate.rs:309 |

## Dispositions

Round 1, reviewed at b1bfa8d32587e558ee1b3f1c1bdf6a028a4278ab (fix commit 08ec6a0, then the merge of main 04eb1e6, which contains #278). `quire coverage --scope .` (quire 0.36.1): main 04eb1e6 314/442, FR-015 39/68, 44 unbacked rows. Head 320/442, FR-015 45/68, 44 unbacked rows. `status_lies` is empty on both, and both have the same 10 suspicions. The +6 are exactly AC-59 and AC-61 to AC-65. AC-60 and AC-68 have status `untagged` with no binders, and no `.rs` file carries their trace tag. All 11 arm mutants re-run on the head are killed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 08ec6a0. The two tests now trace `TC-025` only. AC-60 and AC-68 stay PLANNED, and the AC rows, tests.md:25, TC-025 and interface-001:104 all state the same reason (crate-private role function, no V2 builder under src/, byte identity by construction). The count and the status agree |
| FND-002 | fixed | 08ec6a0. The AC-63 row in FR-015 now states that `MemberAbsent` and the invalid-body, incomplete and over-budget lowering records are covered only through the AC-66 mapping test, which matches tests.md:24 |

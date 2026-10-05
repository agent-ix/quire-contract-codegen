---
id: "SR-1504"
title: "CG PR 277 gap analysis: FR-015-AC-66 and AC-67, the FR-015 refusal-to-record table, TC-025 steps 19 to 28 and interface-001"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@37a7e1e21162ed1af6031fa03dd0464ad4da62e7; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, spec/core/functional/interface-001-codegen-api.md, src/kani/generate/{outcome,frame}.rs, tests/it/kani_obligations_state_frame.rs (diff origin/main...HEAD, merge base e526390)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: references
---

# SR-1504: CG PR 277 gap analysis

## Summary

Ticket: IR-461 (code PR 1 of 2). PR: agent-ix/quire-contract-codegen#277 at 37a7e1e, diffed
against origin/main e526390. This is PR-scoped. I checked FR-015-AC-66 and FR-015-AC-67, the
FR-015 refusal-to-record table, TC-025 steps 19 to 28 and interface-001 against the diff. Code
review is SR-1503 and the spec-review base checklist is SR-1505.

Matrix verification (`quire coverage --scope . --json`, quire 0.36.1, measured on both trees):

- Main e526390: 301/442 rows backed. Head: 303/442.
- The only per-criterion changes are FR-015-AC-66, from untagged to tagged by
  `tests::tc_025_every_state_frame_refusal_maps_to_the_disposition_the_table_gives`
  (src/kani/generate/outcome.rs), and FR-015-AC-67, from untagged to tagged by
  `tc_025_a_non_identifier_graph_field_name_is_a_malformed_clause`
  (tests/it/kani_obligations_state_frame.rs).
- FR-015-AC-59 to AC-65 and AC-68 stay untagged and keep `PLANNED (IR-461)`. No planned AC
  counts as backed.
- `status_lies` is empty on the head. There are no new unbacked rows, untracked symbols or
  unmatched tags against main. The other tsv differences are line shifts from the matrix row
  split.
- The tests.md split, with AC-59 to AC-65 and AC-68 still Planned and AC-66 and AC-67 Covered,
  matches the tags.

TC-025: steps 26 and 27 are the ones the first change backs, and their tests exist. Steps 19 to
25 and 28 belong to PR 2 and are unchanged. Line 229 says the first change "has landed". That
becomes true when this PR merges.

interface-001:104 still lists the StateFrame item and its five reasons as `planned (IR-461)`.
That is still true, because the arm is PR 2. interface-001:107 and :111 name
`StateFrameRefusal` as the output and give no shape, so the payload changes need no edit there.

Reverse gap: the new code (`state_frame_disposition`, `lowering_refusal`, `field_range`,
`range_literals` and the new types) is owned by FR-015:308-354. I found no unowned behaviour and
no stubs.

Plan completion: not assessed. This is a planless, PR-scoped run. Semantic review (whether
intent, test and code agree) was done for AC-66 and AC-67 only, as the dispatching brief asked.

## Verdict

CONDITIONAL. One medium finding. AC-66 is backed: all 29 rows are built directly with exact
dispositions and reasons, the serialised JSON is asserted, and the function text is inspected.
The weakness of that inspection is recorded in SR-1503 FND-001.

AC-67 is backed for the condition-read path only. The reclassification at the frame grant is
reachable and untested (FND-001 below), so the author's ambiguity (5), that it is untestable,
is rejected.

Author's ambiguity (1) is accepted. The merged sentence offered only "the field, and the
member's value.target node when it has one". From that payload alone the mapping could not
separate an unbounded type from a bound that is not an `integer_range`, unless it read the graph
again. A cause enum is the smallest payload that can implement the table. The edited sentence
agrees with table rows 326-327 and with `BoundNotResolvedCause`, and no AC changed.

Author's ambiguity (2), an unreadable `integer_range` body classed as `NotIntegerRange`, is
accepted. It reaches the same table row ("a bound that is not an `integer_range`") and the same
disposition as the other grounds. The doc comment says "a readable `integer_range`".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015:349 and AC-67 say that any field name read from the graph that is not a Rust identifier is `MalformedClause`. The frame-grant case is reachable and no test covers it. The AC-67 test's doc comment says IR's admission check means "no admitted package reaches the engine's own check". That is wrong. IR's frame check (quire-contract-model `checked_package/v2/identity.rs:568` `is_identifier`) admits any ASCII `[A-Za-z_][A-Za-z0-9_]*`, which includes Rust keywords. `syn::Ident` refuses keywords. I ran a probe in my own worktree with the fixture's `Shape { condition_field: "type", modifies: &["type"], .. }`: the package was admitted and the engine returned `MalformedClause { at: <frame node> }`. Mutant M5, which reverts frame.rs:483 to `InvalidField { name }`, passes every test. Add that keyword case to the AC-67 test, asserting `MalformedClause` at the frame node, and correct the doc comment and the tests.md wording | src/kani/generate/frame.rs:483, tests/it/kani_obligations_state_frame.rs:782-790, spec/kani/matrix/tests.md:25 |

## Coverage

- Rows backed: 303/442 at head, against 301/442 at main. The +2 are FR-015-AC-66 and
  FR-015-AC-67 only.
- Examined: FR-015-AC-66 and FR-015-AC-67 are backed. FR-015-AC-59 to AC-65 and AC-68 are
  planned and correctly unbacked. I also examined the FR-015 refusal table (17 variants, 6
  `NotLowered` arms, 5 grounds, 4 effects), TC-025 steps 19 to 28 and interface-001:104-111.
- Semantic review: AC-66 and AC-67 only.
- Plan completion: not assessed

## Dispositions

Round 1, reviewed at dbff533d8a3e7ce1f27608325af7f0f922b7899b.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | dbff533 |

Note for round 1: the AC-67 test now asserts `MalformedClause` for a keyword (`type`) as a
condition read, at the read node, and as a frame grant, at the frame node. The false doc comment
is corrected. The tests.md cell now states what is tested and what is not: four of the five
grounds at engine level, `MemberAbsent` through the mapping only, and no test for the dangling
operand. Mutant M5 (frame.rs:483 reverted to `InvalidField`) is now killed by the AC-67
integration test. Coverage is unchanged at 303/442. The new engine-ground test is also tagged
FR-015-AC-66, so AC-66 now has three binders.

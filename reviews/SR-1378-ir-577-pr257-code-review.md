---
id: "SR-1378"
title: "CG PR 257 code review (with rust-review lane): NFR-005-AC-6 to AC-8, the four IR-577 index and subtraction sites"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@4b5490b1292b963fd89b2bc804b660e4878c9e60; src/routed/generate.rs, src/evidence/bound_coverage.rs, src/oracle/boolean_v1.rs, src/core/diagnostic.rs, tests/it/no_generation_panics.rs (diff origin/main...HEAD)"
---

# SR-1378: CG PR 257 code review

## Summary

Ticket: IR-577. PR: agent-ix/quire-contract-codegen#257 at 4b5490b, one commit on origin/main.
The `rust-review` lane is folded into this file.

What I checked, independently of the PR body:

- The four sites. `generate_kani` no longer builds `driver`; it calls
  `rewrite_duplicate_position`, which reads `group.get(*first_index)` and returns
  `KaniDuplicatePositionOutOfRange { first_index, items: group.len() }` otherwise.
  `observe_clause` destructures `let [_, evaluation_region, consequent_regions @ ..] = map else`
  and pushes the `MapMismatch` diagnostic `typed implication census differs from map`, returning
  the row before anything is observed. `generate_boolean_oracle_inner` reads lines through
  `checked_sub(1)` then `get` and builds the clause region by struct update. Each matches its
  NFR-005 behavior-table row.
- Happy path. For a line `l >= 1` that exists, `l.checked_sub(1).and_then(|at| lines.get(at as
  usize))` reads the same element as `lines[l as usize - 1]`; the struct update yields the same
  `SourceRegion` as the old post-assignment; the region order (clause, evaluation, consequents)
  is unchanged. In `generate_kani` the map closure does the same three steps in the same order;
  the `Result` collect short-circuits only on the new refusal, so valid inputs give the same
  `Vec`. A map of exactly two regions gives `consequent_regions` empty, as `map[2..]` did.
- Mutation, in a throwaway worktree with its own target dir, reverted per mutant. Killed:
  restoring `group[*first_index]` (AC-6 test panics), `map[1]`/`map[2..]` (AC-7 test panics),
  `source_lines[offset as usize - 1]` and `regions[0].expected_consequents` (AC-8 body scan);
  dropping `?`, keyword list, `->` exclusion, `]`/`}` from operand-ending, the subtraction rule
  or the index rule from the scan (AC-8 rule test); `items: *first_index`, `first_index:
  group.len()`, a rewrite to the wrong value, the wrong `MapMismatch` text, and no diagnostic
  (AC-6/AC-7 tests). Survivors: FND-001 and FND-002 below; an off-by-one in `probe_at`
  (`checked_add(0)`) survived the full default suite, but probe columns derive from line
  indentation, so that mutant is not distinguishable on the fixtures and the equivalence above
  was established by reading; the `?` on the consequent probe swapped for `.ok()` survives too,
  on the unreachable path the spec says has no fixture (NFR-005 Verification).
- Scan rules against NFR-005-AC-8: operand-ending tokens `)` `]` `}` `?`, numeric literal,
  non-keyword identifier; the twelve keywords are exactly the spec's list; `->` excluded; `-=`
  flagged; literals are removed by `non_test_code_outside_literals`. All nine positive and seven
  negative cases of TC-042 step 8 are asserted, plus four more unary-minus shapes. A `[` after a
  lifetime (`&'a [T]`) would be flagged; that is the spec's own rule and fails closed. The dated
  `fn digest` exception and AC-1 scan are untouched apart from `digest_body` now delegating to
  the extracted `braced_body`, same algorithm.
- rust-review: no new `unwrap`/`expect`/panic macro in non-test code (AC-1 scan passes). The one
  new `as` cast is `at as usize` on a `u32`, widening on every supported target, same as the
  expression it replaced. The new variant follows `KaniRecordCountMismatch`'s shape and docs;
  the enum carries no `Display`, as before. `make fmt-check`, `make lint`, `make audit-unsafe`,
  `make rustdoc`, `make deny` and `make spec` exit 0 on head; the full default `cargo test`
  passed under each surviving mutant, and the TC-042 tests pass on head.

## Verdict

Mergeable. AC-6, AC-7 and AC-8 are each asserted by a test that fails under the mutation that
reintroduces the site. The refusals carry the spec's fields and leave nothing generated. Two low
test-strength findings, neither blocking.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The AC-8 body scan asserts only that `function_body` returned `Some`; a locator that returned an empty or wrong range would make the scan vacuous and still pass (mutant `return Some(0..0)` survived). Assert the located body is non-empty and contains a known call of each function (for example `rewrite_duplicate_position(` in `generate_kani`, `consequent_regions` in `observe_clause`) | tests/it/no_generation_panics.rs:139-153, tests/it/no_generation_panics.rs:313-338 |
| FND-002 | low | FR-022's whole-call refusal ("the generator shall refuse the whole call") is asserted only at the helper; `generate_kani` discarding the helper's `Err` (`let _ = rewrite_duplicate_position(..)`) survives every test. A bare dropped `?` is caught by `unused_must_use` under `make lint`, so only the explicit discard escapes | src/routed/generate.rs:334 |

## Dispositions

Round 1, reviewed at 8b1152d6d41a24bf0aed34bc5eca52154d23c5f9. The branch was rebased onto
origin/main cc5d6e6 (#256) and force-pushed, so 4b5490b is gone; `git range-diff` shows the
rebased first commit 7159507 is patch-identical to 4b5490b, and the fix round is b41b95a, f2e79e7
and 8b1152d. Mutation in a throwaway worktree with its own target dir: `let _ =` on the
`rewrite_duplicate_position` call in `route_records`, deleting that call, an index or a
subtraction added to `route_records`, and `return Some(0..0)` in the body locator are each killed.
The remaining hop, `route_records(..)?` in `generate_kani`, survives only a type-changing
`.unwrap_or_default()`, which is a deliberate rewrite and not a dropped refusal. The
`route_records` extraction is a verbatim move of the closure, with `harnesses` passed as
`&mut`; TC-033 and the TC-042 tests pass on head, and fmt-check, lint and spec exit 0.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | b41b95a |
| FND-002 | fixed | b41b95a |

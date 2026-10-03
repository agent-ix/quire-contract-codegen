---
id: "SR-1021"
title: "CG PR 243 gap analysis: FR-018-AC-17..19 and FR-014-AC-39 against tests and code"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@0573ebdcff3fc0bca4ad2815961813c5fe8d9943; spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md, src/oracle/equality/mod.rs, tests/common/panic_scan.rs, tests/it/composite_equality_generation.rs, tests/it/exact_scalar_generation.rs; diff origin/main...HEAD, base 4b6e09c"
---

# SR-1021: CG PR 243 gap analysis: FR-018-AC-17..19 and FR-014-AC-39 against tests and code

## Summary

Ticket: IR-538. PR: agent-ix/quire-contract-codegen#243 at 0573ebd. Plan completion: not assessed.

Tests mapped to criteria:

- **FR-018-AC-17:**
  - `tc_029_ac17_emitted_source_has_no_panic_site_fallback_or_index_expression` scans for panic tokens, fallbacks and index expressions. It also asserts the corpus crate has more than one oracle.
  - `tc_029_ac17_emitted_reconstruction_is_typed_and_refused_never_unwrapped` checks the structure. It also requires that all six `rebuild_*` helpers are present in the corpus crate, which makes it non-vacuous.
  - `tc_029_ac17_the_structural_checker_names_each_departure` runs the checker on hand-written bad snippets.
- **FR-018-AC-18:** two unit tests in `oracle::equality::tests`.
- **FR-018-AC-19:** `tc_029_ac19_the_equality_generator_source_holds_no_panic_token`.
- **FR-014-AC-39:**
  - `tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site`.
  - The scan inside `generate()`. Every TC-024 crate that succeeds goes through it; the only direct call is the SourceTooLarge error case, which produces no crate.
  - The scanner self-test `tc_024_ac39_the_panic_scan_names_every_banned_token_and_no_lookalike`.
- **FR-021-AC-21 (source half):** the rewritten `oracle_generators_have_no_panicking_arms` now excuses nothing.

Mutation probes ran in a detached throwaway worktree, with each mutant reverted. Runs used `--no-fail-fast` where a lib-test failure would otherwise hide the `it` binary.

| Mutant | Result |
| --- | --- |
| M1: `.expect(..)` in the emitted `rebuild_integer` template | killed: ac17 scan, ac19 |
| M2: `left_target.clone().unwrap_or(left_source.clone())` in the emitted oracle | killed: ac17 fallback check |
| M3: inline `rt::TextType::new(..)` outside a helper | killed: ac17 structural |
| M4: re-add `ValueType::Quantity => unreachable!(..)` | killed: ac18 unit, ac19, oracle_generators_have_no_panicking_arms |
| M5: `CardinalityBound::new(..).or_else(\|_\| CardinalityBound::new(0, u64::MAX))` inside `rebuild_cardinality` (silent widen) | **survived** (see SR-1022 FND-001) |
| M6: oracle `Err` arm for `left_source_*` returns `Outcome::Completed(false)` | killed: ac17 structural |
| M7: environment `Err` arm yields `Vec::new()` | killed: ac17 structural |
| M8: `rebuild_text` maps to `ReconstructionError::Integer` | **survived** (see SR-1022 FND-001) |
| M9: `settle_render` hardcodes `node_tag: "quantity"` | killed: ac18 boundary test |
| M10: the item loop settles a failed render onto `claims[0]` instead of its own claim | **survived** (FND-001 below) |

Author's flagged ambiguities:

1. **Per-item `*_source_*`/`*_target_*`/`composites_*` as reconstruction helpers.** The tests take the strict reading on each clause. Return types and call handling are checked for both the `rebuild_*` helpers and the per-item functions. Constructor placement allows only `rebuild_*`. Either reading of the AC passes, so this is acceptable.
2. **`rt::Outcome::Refused` versus the bare names.** The checker matches the substrings `Outcome::Refused(` and `Refusal::CheckedInvariant`, which cover the `rt::` path. Acceptable.
3. **"No code emitted" checked only at unit level.** Acceptable, because FR-018-AC-18 itself directs a unit test (the path is unreachable through the public API). The same unit-level placement, however, leaves the sibling clause asserted only tautologically (FND-001).

Matrix: see SR-1022. Gates were measured here. `cargo fmt --check` and `cargo clippy --locked --all-targets -- -D warnings` are clean. The focused filters pass (90 `it` tests and 4 lib tests in the touched modules). The full `cargo test --locked` run passes: 117 lib tests, 261 `it` tests with 15 ignored (the real-prover Kani tests), and 1 more.

## Verdict

AC-17, AC-19 and FR-014-AC-39 are strongly backed: every planted panic, fallback, inline constructor and silent `Err` arm was caught. AC-18's typed-error and refusal-mapping clauses are backed. Its "leaving its siblings unchanged" clause is not: the test's sibling is never handed to the code under test, and M10 shows that refusing the wrong claim passes every test. Fix that, or mark the AC-18 row partial with the reason, before the row says Covered.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-18 "leaving its siblings unchanged" is asserted tautologically. `sibling` is never passed to `settle_render`, so `assert_eq!(sibling, generated_claim('2'))` compares a value with itself. Sibling isolation lives in the item loop (`settle_render(&mut claims[claim], ..)`, naming over `rendered`), which no test exercises. M10 (settle onto `claims[0]`) survives every test. | src/oracle/equality/mod.rs:627-636, src/oracle/equality/mod.rs:2096-2115 |
| FND-002 | low | The rewritten `oracle_generators_have_no_panicking_arms` now scans only code before `#[cfg(test)]` for all three generators. FR-021-AC-21, which it traces to, covers the whole of `src/oracle/function/mod.rs` (comment lines excepted). That file has no test module today, so the test is equivalent now but narrower than its AC. | tests/it/exact_scalar_generation.rs:2201-2239, spec/oracle/functional/FR-021-function-application-oracles.md:260 |

## Dispositions

Round 1 was reviewed at d55fdd404a2a7f86e6f62f0ee1d4816e90ceec46 (fix-round delta fff7549..d55fdd4, on top of a clean rebase onto 68ca78c). Mutants ran in a detached throwaway worktree with `--no-fail-fast`, and each was reverted:

| Mutant | Result |
| --- | --- |
| M10: `render_and_emit` settles a failed render onto `claims[0]` | killed by `tc_029_ac18_a_render_failure_refuses_only_its_item_and_never_renames_a_sibling` |
| M5: silent widen via `or_else` in `rebuild_cardinality` | killed by `tc_029_ac17_emitted_reconstruction_is_typed_and_refused_never_unwrapped` |
| M8: `rebuild_text` maps to `Integer` | killed by the same test |
| P1: `x.map(Option::unwrap)` in generator code | killed by tc_029_ac19 |
| P2: emitted `.unwrap ()` in `rebuild_integer` | killed by ac17 (both tests) and ac19 |
| P3: emitted `debug_assert_eq ! (1, 1);` | killed by ac17 scan and ac19 |
| P4: `x.expect_err("p")` in generator code | survived. The AC does not list it; see SR-1022 FND-003 |
| FR-021-AC-21 probe: `#[cfg(test)] mod probe_tests { .. unreachable!(..) }` appended to src/oracle/function/mod.rs | `oracle_generators_have_no_panicking_arms` passes |

Other measurements:

- `quire validate` (spec, plan and reviews) exits 0.
- `quire coverage --strict` reports 66 unbacked, unchanged.
- The four matrix rows still read Covered, which is now true for AC-18.
- The full `cargo test --locked` passes: 118 lib, 267 `it` (15 ignored Kani), 1 other.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d55fdd4 |
| FND-002 | still-open | The scan now uses `non_test_code`, which strips every `#[cfg(test)]` item. FR-021-AC-21 still counts the whole of src/oracle/function/mod.rs, comment lines excepted. A `#[cfg(test)]` module holding `unreachable!` passes the test (measured). Fix: scan function/mod.rs with comments stripped only, or amend FR-021-AC-21 to say non-test code. |

Round 2 was reviewed at 58a6050e9d163c2c03d76c2951f1831121b4d766, a fast-forward from d55fdd4 with main still at 68ca78c. Only the delta d55fdd4..58a6050 was reviewed, in a detached throwaway worktree:

- **The FR-021-AC-21 probe is now killed.** I appended `#[cfg(test)] mod probe_tests { .. unreachable!(..) }` to src/oracle/function/mod.rs, and `oracle_generators_have_no_panicking_arms` fails. The function generator is now scanned with `comments_stripped`, which keeps test items.
- **The new test `tc_031_ac21_the_whole_file_scan_counts_a_test_module_and_not_a_comment`** asserts three things: the test-module macro is named, a commented `todo!()` is not, and `non_test_code` would drop the module.
- **Gates.**
  - fmt-check and clippy `--all-targets -D warnings` are clean.
  - The full `cargo test --locked` passes: 118 lib tests, 268 `it` tests (15 Kani tests ignored), and 1 more.
  - `quire validate` exits 0, and `quire coverage --strict` reports 66 unbacked.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 58a6050 |

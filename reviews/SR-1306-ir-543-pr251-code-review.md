---
id: "SR-1306"
title: "CG PR 251 code review: NFR-005 panic-site removal, scan and TC-042 seams (Rust lane)"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@0b11c77e0408242377eb86e5aac29a9bbac9f07f; src/evidence/bound_coverage.rs, src/kani/generate/corpus/bounded_kani_corpus.rs, src/kani/generate/outcome.rs, src/kani/generate/scalar.rs, src/oracle/boolean_v1.rs, src/oracle/function/mod.rs, src/routed/capability.rs, src/routed/generate.rs, tests/common/panic_scan.rs, tests/it/main.rs, tests/it/no_generation_panics.rs (diff origin/main...HEAD, base dc4166a)"
---

# SR-1306: CG PR 251 code review

## Summary

Ticket: IR-543. PR: agent-ix/quire-contract-codegen#251 at 0b11c77. This is a code review with
the Rust-review lane folded in. Repository conventions take precedence. It checks each of the
fourteen NFR-005 sites for behaviour parity and soundness, and checks the scan and seam tests
for oracle strength.

What I re-measured myself:

- **Independent scan.** I wrote my own lexer, separate from `panic_scan.rs`. It strips
  comments, plain, byte and raw strings, char literals and `#[cfg(test)]` items, then looks for
  every FR-014-AC-39 token, plus `process::exit` and `unwrap_or_else(|| panic!`. Over the 70
  `.rs` files in `src/` it found exactly two places with tokens: `src/kani/test_support.rs`,
  which `src/kani/mod.rs:27-28` declares under `#[cfg(test)]`, and the single dated `expect` in
  `CaseIdentity::digest` (`bounded_kani_corpus.rs:349`).
- **Mutations to the scan.** Each was applied in a throwaway copy and reverted. Every one was
  killed by `tc_042_ac1_src_holds_no_panic_token_outside_the_dated_digest_exception`:
  - an `unwrap()` added to a clean file (`src/core/artifact.rs`)
  - a second `expect` added inside `digest`
  - an `expect` added in `render_artifacts`
  - the `expect` in `digest` changed to `unwrap`
- **Mutations to the behaviour.** Every one was killed by its seam test:
  - both `lower_scalar_claim` branches turned into a panic: killed by the unit test and by the
    integration test through `negotiate_kani_obligations`, so the integration test really
    reaches both branches
  - the probe message changed
  - classification allowed while diagnostics are present
  - the probe checked before coverage
  - a wrong Subtract range
  - a two-corner Multiply
  - `pair_records` accepting fewer records
  - a warning returned for an `invalid_capability` cause
- **Emitted-output parity.** I patched `Artifact::new` (throwaway) to dump every artifact built
  by the whole non-ignored suite (`cargo test --locked`), at base dc4166a and at head 0b11c77.
  Both runs produced the same set of 5434 distinct (path, contents) artifacts, byte for byte.
  That set includes the scalar Kani harnesses, the corpus artifacts and the function-oracle
  crates. The refactors change no emitted text on any path the suite exercises. The ignored Kani
  lanes generate through the same functions.
- **Gates at head**, all exit 0, with free disk at 431G: `make fmt-check`, `lint`, `test` (130
  lib, 284 it with 15 ignored, 1 doc), `deny`, `audit-unsafe`, `rustdoc`, `msrv` and `spec`.

Per-site parity, all sound:

- **(a) `generate_exact_function_oracles`.** `Stage1Shape` carries body, parameter kinds and
  result kind. The resolution loop zips `ordered_functions` with `shapes` by position, and
  Stage 2 consumes them. `resolve_operand_type` is a pure function of the same inputs, so the
  carried kinds equal the re-resolved ones. Refusal order is unchanged: the Stage 1 closure is
  untouched, and `resolved` is still keyed by node id with first-wins. `TypeEnvironment` derives
  `Default` as two empty maps in quire-contract-runtime ccc722b. There is no remaining
  `[index]`. One edge: if `lowering.records` were shorter than the requested list, the old code
  panicked; the new one falls through to the existing `BodyMismatch` fallback, which is a typed
  refusal.
- **(b) `ScalarOperands`.** The formulas are unchanged. Operands are read in position order 0
  then 1, as the old `0..operand_names().len()` map did, so the first error wins in the same
  order. `ranges()` reproduces the old `Vec` order. `i128` cannot overflow on `i64` corners.
- **(c) `lower_scalar_claim`.** Both sites now return `Err(NoRenderer)`. The rewritten docs on
  `ScalarLoweringRefusal`, `NoRenderer` and `OperationNotRendered` match the NFR-005 row and
  TC-025.
- **(d) `ItemSettlement::warning`.** It returns `None` for the six named causes. The match is
  still exhaustive and has no catch-all, and the doc states the `None` case.
- **(e) `observe_clause`.** Coverage is still checked first. A missing probe gives `MapMismatch`
  "missing semantic probe" for both regions. Classification happens only on `Ok` with empty
  diagnostics. For complete maps the behaviour is unchanged: any `Err` evaluation already pushed
  a diagnostic.
- **(f) `render_artifacts`.** It is pure and does not touch `emitted`. Rendering before `claim`
  only changes precedence when serialization fails, which is unreachable today. A refusal
  records no identity. There is a single caller, it uses `let ... else` with no unwrap, and
  artifact order and paths are unchanged.
- **(g) `pair_records`.** It returns `KaniRecordCountMismatch { records, items }`, matching
  interface-001:139-140. The enum has no Display, serde or `From` impls, consistent with its
  sibling variants. No exhaustive match on `RoutedGenerationError` exists outside tests.
- **(h)** The `boolean_v1` `debug_assert!` is removed.

## Verdict

The change is correct and behaviour-preserving, and the seam oracles are strong. Two low
findings, neither blocking.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `MUST_SCAN` is a `[&str; 10]` that names `src/oracle/function/mod.rs` twice, at index 0 and index 9, so the vacuity guard checks nine distinct files while it reads as ten. Nothing required is missing: the seven measured-site files plus the three FR-014/018/021 files are nine distinct paths, because `function/mod.rs` is in both lists. Drop the duplicate and size the array 9, or keep 10 entries with a comment that one file is in both lists | tests/it/no_generation_panics.rs:26-37 |
| FND-002 | low | `tc_042_ac1_operands_of_each_operation_have_its_own_arity_and_reach_exact_extremes` says "exact extremes", but it uses only the ranges (-2,3) and (4,5). No operand at `i64::MIN` or `i64::MAX` is exercised, so it does not show that `reachable` stays exact at the `i64` boundary: `-i64::MIN`, and `i64::MIN * i64::MIN` in `i128`. The formulas are unchanged and the emitted-artifact parity run matches. Add one boundary case per operation, or rename the test | src/kani/generate/scalar.rs:783-801 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | No test pins the `b * c` corner of Multiply's range. A mutant that drops it (`let corners = [a * c, a * d, b * d];`) passes the renamed `tc_042_scalar_operands_have_their_own_arity_and_reach_exact_extremes` and passes the whole non-ignored suite (130 lib, 284 it, 1 doc; run at 8216fe4). The reason: in the small case `b * c` (12) is neither extreme, and in the full-range case `b * c == a * d`. `reachable` is the only direct oracle for the scalar harness's reachable range, so an asymmetric Multiply case would go uncaught. Fix: add one case where `b * c` is the only extreme, e.g. left (1, 2) and right (-5, -4), which gives (-10, -4) | src/kani/generate/scalar.rs:782-808 |

## Dispositions

Round 1, reviewed at 8216fe4cdda057953e1191dbdd15118a5dad7c9d.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 8216fe4 | 8216fe4cdda057953e1191dbdd15118a5dad7c9d |
| FND-002 | fixed 8216fe4 | 8216fe4cdda057953e1191dbdd15118a5dad7c9d |

Round 2, reviewed at a7755532ae2fe4520daf8ebd2e2561dfae02b2da.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed a775553 | a7755532ae2fe4520daf8ebd2e2561dfae02b2da |

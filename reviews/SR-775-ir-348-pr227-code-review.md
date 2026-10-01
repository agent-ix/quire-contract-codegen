---
id: "SR-775"
title: "CG PR 227 code review: AD-004 step 2d, oracle/ module move"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@8692a13a50fd56ab35e4ebbab4bc264ae47bd264; src/ (the 13 files the PR touches: lib.rs, oracle/{mod,boolean_v1,bound_v1,claim}.rs, oracle/{scalar,equality,function}/mod.rs, kani.rs, kani_obligations.rs, state_frame.rs, harness.rs, bound_strategy/generation.rs), tests/ (comments naming moved files)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-775: CG PR 227 code review (code-review with the rust-review lane)

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#227 at 8692a13 (move commit 594e786, plus one
comment-path fix). Base: origin/main ea07b79. A pure-motion PR, so the review checks that nothing
but paths changed, that output and public API are identical, and that the new module layout is
idiomatic. Rust lane folded in, as `rust-review` says.

Measured by the reviewer:

- Pure motion. `git diff -M --summary`: `oracle.rs -> oracle/boolean_v1.rs` (100%),
  `generation.rs -> oracle/claim.rs` (100%), `bound.rs -> oracle/bound_v1.rs` (98%),
  `exact_scalar.rs -> oracle/scalar/mod.rs`, `composite_equality.rs -> oracle/equality/mod.rs`,
  `exact_function.rs -> oracle/function/mod.rs` (99% each). 13 files, +53/-44 (the new
  `oracle/mod.rs` is 18 of the insertions). Every changed line in a moved file or importer is a
  `use` line, an intra-doc link, or a comment path. Nothing renamed, reordered or reformatted
  except the rustfmt-forced sort of `lib.rs`'s `pub use` lines and the merge of `bound_v1.rs`'s
  two imports from `boolean_v1` into one `use`. 594e786 to 8692a13 changes one comment line.
- Root-path imports in moved files. Every `use` in the six moved files is now a `core::` or
  `oracle::` module path (`bound_v1.rs`'s `OracleArtifactBundle, OracleRequest` root import was
  rewritten). The three remaining `crate::<Item>` mentions are intra-doc links
  (`boolean_v1.rs:22`, `claim.rs:30`, `claim.rs:74`), which AD-004 leaves to 2g-0. Test modules
  use `super::*`.
- Visibility. `oracle/mod.rs` declares the six modules `pub(crate)`. At base they were private
  `mod`s of the crate root, already visible to the whole crate, so `pub(crate)` is the minimum
  that keeps the sibling importers compiling and widens nothing. No item visibility changed. The
  only `pub mod` is still `bound_strategy` (step 2e).
- Public API. The crate-root `pub use` name set is unchanged (244 names, same set at base and
  head). rustdoc `all.html` built at base and head with `cargo doc --locked --no-deps`: byte
  identical, sha256 `69b151c9...4ac1`, same 286 item links; no warnings either side. No shim and
  no `pub use` of an old path.
- Byte identity, reproduced independently. Uncommitted hook in `Artifact::new` (path and
  contents) and `ArtifactBundle::new` (bundle serde JSON) in three scratch worktrees outside the
  repo (base ea07b79, move 594e786, head 8692a13), `cargo test --locked` in each. Results in
  Verdict.
- Direction. `oracle/*` imports only `core::` and `oracle::`. Importers of `oracle::` are
  `kani`, `kani_obligations`, `state_frame` (kani) and `harness`, `bound_strategy/generation`
  (strategy): all allowed by AD-004 (`kani --> oracle`, `strategy --> oracle`). No cycle.
- Trace tags. The `// Implements: FR-014/FR-018/FR-021` lines moved from `lib.rs` to
  `oracle/mod.rs` with their modules; `quire coverage` registers them on the new path (see
  SR-776). Unit tests moved unchanged inside their files.
- Gates. `make ci` exit 0 and `make kani` 9 passed at 8692a13 are the coder's; the kani log
  (`cg-2d-kani.log`, `head=8692a13... exit=0`, 9 passed) was read. Kani was not re-run: a pure
  `git mv` plus path edits, with output byte identity reproduced on the final head, changes no
  harness text.

## Verdict

PASS with one low finding. The move is pure, the public API and generated output are
identical, and the module layout is idiomatic. FND-001 is a cheap in-PR fix; FND-002 is a nit.
Mergeable once FND-001 is fixed (or explicitly deferred by the lead).

Byte identity: `cargo test --locked` passed 115 unit, 246 integration (9 ignored, the Kani lane)
and 1 doc test on all three trees, so the test count is unchanged. The sorted, hex-encoded dumps
hold 7,174 records each (7,095 artifacts, 79 bundles; 165,198,553 decoded payload bytes),
sha256 `e768024a...22f5`, and `cmp` finds base == move and base == head. The record count
matches the coder's. The coder's byte and sha figures cover a different encoding, so they are
not comparable. The reviewer's run covers the final head, which the coder did not re-dump.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Seven comments in `tests/` still name `src/exact_scalar.rs` or `src/exact_function.rs`, which this PR removes. The same PR fixed the identical kind of comment in `src/` (8692a13, `kani_obligations.rs:2384`), and AD-004 makes 2c to 2g "`git mv` plus path fixes" without deferring test comments to any later step (2g-0 covers root imports and doc links, step 7 covers spec). Repoint them to `src/oracle/scalar/mod.rs` and `src/oracle/function/mod.rs`. | tests/it/exact_scalar_generation.rs:665, tests/it/exact_scalar_generation.rs:702, tests/it/exact_scalar_generation.rs:1170, tests/it/exact_function_agreement.rs:18, tests/exact_function_support/agreement_cases.rs:19, tests/exact_scalar_support/agreement_cases.rs:141, tests/exact_scalar_support/package.rs:799 |
| FND-002 | low | The new `oracle/mod.rs` module header says "The files are moved from the flat layout unchanged (AD-004 step 2d)", and the `lib.rs` comment says "The oracle subsystem (AD-004 step 2d)". Migration history in a permanent module header goes false at step 3, when `scalar`, `equality` and `function` move onto `core/ir`. Say what the module owns and drop the history clause. | src/oracle/mod.rs:4-5, src/lib.rs:21 |

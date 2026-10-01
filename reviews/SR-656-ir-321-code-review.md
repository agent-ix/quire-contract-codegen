---
id: "SR-656"
title: "CG PR 213 code review: doc-comment spec paths after the subsystem restructure"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@b17b2b171fc52500c03710239a7cba5abc0e2f1f; src/exact_function.rs, tests/it/exact_function_generation.rs, tests/it/exact_scalar_generation.rs, CLAUDE.md"
---

# SR-656: CG PR 213 code review

## Summary

Ticket: IR-321. PR: agent-ix/quire-contract-codegen#213 at b17b2b1, base baab597.
The code side of this restructure is four doc-comment path edits and one CLAUDE.md layout line.
The rust-review lane is folded in here; no Rust logic changed.

Examined:

- `src/exact_function.rs:111`: `spec/test-matrix.md` became `spec/oracle/matrix/tests.md`
  (FR-021-AC-15 row).
- `tests/it/exact_function_generation.rs:224`: the FR-018 row (AC-7 refused-forms note).
- `tests/it/exact_function_generation.rs:690`: the FR-021 row (AC-15).
- `tests/it/exact_scalar_generation.rs:639`: the FR-014-AC-14 row.
- `CLAUDE.md:51`: the spec layout line.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean.

- `git grep` for `spec/test-matrix.md` at baab597 in src/, tests/, plan/, Makefile, .github/ and
  scripts/ finds exactly these four places. At b17b2b1 no file outside reviews/ names an old spec
  path (spec/index.md, spec/functional, complete-v1, strategies, spec/reviews,
  spec/evidence/suites.md, spec/interface, spec/stakeholder, spec/nonfunctional, spec/test/).
- Each new target exists and holds the row the comment cites. The FR-018 AC-7 note, the FR-021
  AC-15 row and the FR-014 AC-14 row are all in `spec/oracle/matrix/tests.md`.
- The CLAUDE.md line matches the tree: `spec.md`, `tests.md`,
  `<subsystem>/{stakeholder,functional,non-functional,matrix}/`, and
  `spec/core/matrix/suites.md`.
- The ticket says FR-003 is `include_bytes!`'d in `src/kani.rs` and that interface-001 is read by
  `tests/it/interface_001.rs`. Neither is true at baab597. No FR-003 file exists. Every
  `include_bytes!`/`include_str!` in src/ and tests/ points at schemas/, tests/fixtures/ or
  tests/state_frame_support/. `tests/it/interface_001.rs` does not exist. No source, test,
  Makefile or build file reads a spec/, plan/ or reviews/ path.
- The Makefile and `.github/workflows/ci.yml` validate `spec/**/*.md`, `plan/**/*.md` and
  `reviews/**/*.md`. Those globs still cover the new tree.
- `cargo fmt --all --check` passes. No build was needed: the comments are plain `//!` and `///`
  text with no intra-doc links.

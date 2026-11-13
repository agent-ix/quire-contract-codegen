---
id: SR-17940486709564847047901
title: "IR-1526 PR #814 tractability refusal code review"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #814 @ e02f9053; IR-1526; build.rs, src/kani/tractability.rs, src/kani/generate/clause/v2.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/kani/mod.rs, src/lib.rs, src/oracle/clause.rs, src/kani/run/execute/two_phase_tests.rs, tests/it/kani_tractability.rs, tests/it/kani_obligations_v2_clause_controls.rs, tests/it/main.rs, tests/state_frame_support/native_twin.rs, tests/fixtures/tractability/corpus.json, tests/fixtures/tractability/baseline.json, spec/kani/functional/FR-056-kani-tractability-refusal.md, spec/kani/matrix/TC-070-kani-tractability-refusal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
---

## Summary

Ticket: IR-1526. Code review (with the rust-review lane folded in) of PR #814, scoped to
`git diff origin/main...HEAD` (17 files, +1911/-14). The PR adds the `TractabilityDecider`
seam, the JSON-loaded `TractabilityTable`, `UnsupportedObligation::V2ClauseNotTractable`, a
per-operation width export, `negotiate_kani_obligations_with_options`, a build script for the
kernel version, the corpus and baseline, and host tests for FR-056-AC-1 to AC-12.
`cargo fmt --check` is clean. The focused `tc_070` tests were run on the host with a throwaway
target dir. Clippy, make ci and Kani were not run (the brief excluded them).

## Verdict

**FAIL** - one high finding. The decider seam, the loader, the lookup and tie-breaks, the
staleness rule, the refusal fields and the sibling rule are correct and well tested. The decider
runs after both operator checks, and `V2ClauseNotTractable` reaches the record through the
existing `UnsupportedObligation` arm. No other lane's files are touched, nothing is vendored, and
the corpus cites only CG's own `docs/qualification/` records. The high finding: the
motivating case, a wide integer multiply, can never reach the decider.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high     | A wide integer multiply, the case FR-056 exists for, is never refused. Widths come from the native kernel's selection, which has no interval for a multiply over operands wider than about 31 bits. `refuse_if_not_tractable` then returns Run, so even a current 128-bit `not_tractable` multiply row lets a 41-bit multiply through, and a test pins that behaviour. The PR's Status "Known limit" note narrows the Lookup SHALL rather than meeting it. An i128 interval propagation would give these widths without the native capacity limit. | src/kani/generate/clause/v2.rs:312-334, src/oracle/clause.rs:784-819, tests/it/kani_tractability.rs:438-453 |
| FND-002 | medium   | `build.rs` reads `KERNEL_VERSION` from CG's own `Cargo.lock` at `CARGO_MANIFEST_DIR`. (a) When CG is built as a git or path dependency, this is CG's lock, not the consumer's resolution, so it is not reliably "the quire-exact crate CG links". (b) The build panics when no lock sits beside the manifest. (c) It hand-parses TOML, assuming the `version` line follows `name` and taking the first match. The value is `0.1.0` on every quire-exact `main` commit, so the kernel half of the AC-7 staleness rule never trips. A `VERSION` const exported by quire-exact needs no build script. | build.rs:1-29, src/kani/tractability.rs:357-359 |
| FND-003 | low      | `TractabilityRow.wall_seconds` is `u64`. A fractional value (CG's qualification records and its own corpus use values like 44.6) makes the row malformed and fails the whole generation request. The corpus types the same field as `f64`. | src/kani/tractability.rs:483, tests/it/kani_tractability.rs:612 |
| FND-004 | low      | `negotiate_kani_obligations_with_options(request, options, native: bool)` has a bool parameter whose call site (`..., true)`) does not show the choice. The existing API spells native as a separately named entry point (rust-style api-design: enum or named fn over an opaque bool). | src/kani/generate/negotiate.rs:337-347 |
| FND-005 | low      | An operation step with no native selection decision silently gets width 0 (`map_or(Some(0), ...)`). Width 0 is the most refusable width, because any row for that id qualifies. The doc says only Boolean operations take this path, but nothing checks that the operation is Boolean. | src/oracle/clause.rs:804-815 |

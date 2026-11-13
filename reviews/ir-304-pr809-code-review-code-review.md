---
id: SR-17940486709564847047751
title: "IR-304 PR #809 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #809 at 3c7845ca (git diff origin/main...HEAD); IR-304 second PR; src/kani/generate/scalar.rs, tests/it/kani_obligations.rs, tests/it/requirement_records.rs, docs/qualification/ir304-kani-guard-native-harness.md, docs/qualification/ir304-kani-guard-cover-attempt.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304. The PR adds `Statement::Forget("meter")` and `Statement::Forget("outcome")` at
the end of `render_scalar`, so the existing renderer (`src/kani/generate/render.rs`, unchanged)
reads the cover condition into `let cover_reached: bool = completed;`, emits
`core::mem::forget(meter); core::mem::forget(outcome);` and ends with
`kani::cover!(cover_reached, ..)`. The test-only `render_scalar_reference` gets the same three
lines, the pinned cover text in `tc_025_scalar_harness_asserts_the_native_arithmetic_relation` is
updated, and a new test `the_guarded_native_harness_is_thin_and_ends_with_its_cover` builds the
FR-022-AC-35 harnesses through `generate_routed_native` for `<` and `>`.

rust-review lane checks, no defect: `meter` and `outcome` are owned locals (`outcome` borrows
`meter` mutably only for the call, which has returned), so the forgets compile on both routes and
leak nothing outside the proof; `Statement::Forget` passes the spec validator (after the call,
bound names); the cover still observes `outcome` because `completed` is computed before the forget;
`cover_reached` is a plain `bool`; exactly one cover stays last (FR-015-AC-7/53/58). No `unsafe`,
no panic surface, no integer conversion changed. The new test checks the native representation,
no `rt::Integer`, the native operand binding, the guard assumption once and before the oracle
call, and the last four statements in order. Mutants of the earlier aborted review session
(`/tmp/claude-1000/-home-peter-dev/1266cbc5-5a3a-4d9a-8d9d-965107b1a2ef/scratchpad/rv809-m-*.log`):
the forgets removed and the cover moved first each fail the new test and the lib
`scalar_port_*` differential tests; the guard-late mutant run left no result and its edit is
still applied, uncommitted, in `/home/peter/dev/worktrees/IR-304c-review` (not the PR worktree).
By reading the test, that mutant fails `proof.find(&assumption) < proof.find("let outcome = ")`.

Executed-text provenance: the five scratch crates under
`/tmp/claude-1000/-home-peter-dev/1266cbc5-5a3a-4d9a-8d9d-965107b1a2ef/scratchpad/e304/` were diffed
against a dump of the harnesses this head generates (`rv809-gen-lt.rs`, `rv809-gen-gt.rs`). Only
three comment lines differ (package name `ac35lt`/`ac35gt` against `ac35`); the mutant crates
differ from their base only at the named site. The Kani logs show the stated verdicts (lt: 0 of 1
covers, SUCCESSFUL; gt: 1 of 1, SUCCESSFUL; mutop and mutexact: FAILED; mutnoguard: 1 of 1,
SUCCESSFUL). The runner question is recorded in the gap-analysis and spec-review SRs.

Reviewer gate at 3c7845ca (own run, PR worktree, `CARGO_TARGET_DIR=<worktree>/target` under
`flock /tmp/cargo-slot.lock`): see Verdict.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The change rewrites the tail of every scalar harness `render_scalar` emits, the general `rt::Integer` route of ordinary `generate_routed` included, not only the native AC-35 harness. The installed-Kani `make kani` lane holds the existing real-Kani scalar runs over these harnesses (TC-027 routed scalar `x + 1` `Verified` and the narrowed-domain `Falsified`, the native scalar fixture), and the PR gate list runs fmt, clippy, `--lib`, `--test it`, `make spec` and the matrix but not `make kani-scope` / `make kani` / `make kani-gate`. The five scratch-crate runs do not exercise the general route or the falsified-playback path. Run the lane (or show `kani-scope` does not select it) before merge | src/kani/generate/scalar.rs:789-792, Makefile:108-123 |

## Verdict

Reviewer gate at 3c7845ca: whole `cargo test --locked --test it` 955 passed, 0 failed (41
ignored); whole `cargo test --locked --lib` 548 passed, 1 failed (7 ignored). The one failure,
`production_refinement_live_solver_handoff_reaches_qsl_checker`, panicked only because
`QUIRE_CVC5_BIN` was unset in the reviewer shell; re-run alone with it set, it passes. It is
unrelated to the diff. The installed-Kani lane was not run by the reviewer (brief scope).

Not mergeable at 3c7845ca until FND-001 is resolved and the gap-analysis/spec-review findings on
FR-022-AC-35's evidence are fixed. The code change itself is correct and minimal.

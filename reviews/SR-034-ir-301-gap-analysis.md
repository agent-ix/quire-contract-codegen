---
id: "SR-034"
title: "IR-301 gap analysis: routed scalar harness execution"
type: SpecReview
schema_version: "1.0"
scope: "agent-ix/quire-contract-codegen@5f8dd9f; src/kani_execution.rs, src/lib.rs, tests/it/kani_obligations.rs, tests/it/kani_witness_join.rs, spec/functional/complete-v1/FR-017-pinned-kani-execution-evidence.md, spec/functional/complete-v1/FR-022-routed-generation.md, spec/test/complete-v1/TC-027-pinned-kani-execution-evidence.md, spec/test-matrix.md"
relationships: []
---

## Summary

Gap analysis for IR-301 / PR #181, scoped to the diff `origin/main...HEAD`
(`edade59` -> `5f8dd9f`) only — no full-repo plan bundle targets this PR, so this
runs as the mechanical matrix-verification and reverse-gap steps (`quire coverage
--scope . --json`) against the touched files, per the reviewer skill's
manual-check fallback shape.

`quire coverage --scope . --json` was run from the worktree root
(`/tmp/claude-1000/-home-peter-dev/1c021523-1315-469a-97b0-cc0762694a67/scratchpad/coverage.json`,
stderr in `coverage-err.log` — the stderr warnings are pre-existing module/archetype
duplication notices from `spec-artifacts-process` contributing itself twice, unrelated to
this PR and not in any file it touches).

- **`unbacked_rows`**: no row naming FR-017 or FR-017-AC-11. The new AC is backed by a real
  tracking tag, not a claim: `tests/it/kani_obligations.rs` carries `Trace: FR-017-AC-11`
  doc comments on `tc_027_a_routed_scalar_harness_identity_pin_drift_is_refused_before_the_backend_is_measured`,
  `tc_027_a_routed_scalar_harness_is_refused_when_the_installed_backend_drifts`,
  `tc_027_a_routed_scalar_harness_run_classifies_like_a_contract_harness`, and the
  `#[ignore]`d `tc_027_a_routed_scalar_harness_runs_under_real_pinned_kani`, all of which ran
  green in this review's gate run (three of the four; the fourth is the kani lane, correctly
  excluded from this review's gate scope) — see the code-review artifact SR-033 for the exact
  commands, exit codes and log paths.
- **`status_lies`**: none naming FR-017 — the test-matrix row's `✅ Covered` claim for
  FR-017-AC-11 (added in this PR's `spec/test-matrix.md` diff) matches what the tests
  actually assert.
- **`untracked_symbols`**: none in `src/kani_execution.rs` — the new
  `KaniExecutableHarness` enum, its two `From` impls, and the private `HarnessView`
  projection all trace to FR-017 (the requirement this file already owns per its own
  "no owning requirement" note, extended by FR-017-AC-11 in this PR) and to FR-022 (source
  of the exact-scalar harness FR-017 now also runs, declared as an upstream dependency edge
  added in this PR's FR-017 diff). No reverse gap: nothing in the diff is unspecified
  production code.
- **`no_symbol_rows`**: none naming FR-017 or `kani_execution`.

Field-level cross-check (manual, since this is a single-PR diff, not a Test Matrix
row-by-row sweep): `HarnessView::view()`'s two match arms were checked against
`KaniObligationIdentity` and `ScalarObligationIdentity` (`src/kani_obligations.rs` lines
492-534 and 571-601) field by field; every field FR-017-AC-11's text claims is read
("the pins, the identity digest, the source artifact, the oracle-source digest, the runtime
revision, the unwind bound, the solver and the option vector") is actually read in the
corresponding arm, with no silent default or placeholder value.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No gaps found | - |

## Verdict

PASS. FR-017-AC-11 is backed by real tracking tags in passing tests (not merely claimed by
the test-matrix edit); no untracked symbols or unbacked rows touch FR-017 or
`src/kani_execution.rs`; the new `KaniExecutableHarness`/`HarnessView` code traces fully to
FR-017 (extended) and FR-022 (declared upstream), with no underspecified production code
introduced by this PR.

## Coverage

`quire coverage --scope . --json` totals (whole repo, informational only — this PR's own
scope is clean as detailed above): `backed: 222, total: 265, criteria: 223`. This total
includes pre-existing unrelated gaps elsewhere in the repo, out of scope for this PR.

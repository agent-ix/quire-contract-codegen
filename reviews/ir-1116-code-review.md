---
id: SR-2026102763
title: "Code review — quire-contract-codegen PR 653 composite agreement shared target (IR-1116)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; tests/it/composite_equality_agreement.rs, tests/it/exact_scalar_agreement.rs"
review_set: subset
---

## Summary

Ticket: IR-1116. PR: quire-contract-codegen#653, head
3fd3c7806f4bfda5efa41dfcffb4d586652b3c12 against origin/main
6227850874b5891516be6c9fcc56a81a900705c4. The shared-target design is sound and the
rewrite preserves all 15 composite cases and their failure detection, but the patch restates
an exported constant, binds a harness test to an acceptance criterion it does not assert, and
leaves the ticket's aggregate measurement open. The rust-review lane is folded into this file.

## Verdict

**FAIL** — FND-001 is graded `high` by the code-review duplication rule, which forbids softening
it; its practical consequence is small (a generator rename fails loudly) and the fix is one line.
No high-severity behavioural defect was found.

Examined and found correct:

- **Identity rewrite.** `composite_manifest_for_shared_target` changes only the `[package]` name
  and panics when the generated header or the single `name =` line changes. An explicit `[lib]`
  name is refused. Nothing in `tests/composite_equality_support` reads the package identity.
- **Sibling isolation.** A throwaway Cargo probe (removed afterwards) reproduced the hazard: two
  same-named path crates in one target shared one `agreement-<hash>`, and the second crate ran
  the first crate's binary and reported ok with no compile. Distinct names gave distinct rlib and
  test hashes, and each crate compiled. The rewrite is necessary and sufficient for that hazard.
- **Staleness.** `remove_dir_all` plus regeneration gives each run newer source mtimes, so a kept
  target rebuilds the 15 local crates and reuses only dependency units.
- **Case authenticity.** `fixture::CASES`, the alias match and the 15 files under
  `tests/composite_equality_support/cases/` are untouched. Each case `include!`s its own
  `CARGO_MANIFEST_DIR/src/lib.rs` and `names.rs`, so the oracle under test comes from its own
  scratch root.
- **Failure detection.** Exit status and the `passed == declared` count are unchanged.
  `run_agreement_mutant` and `run_agreement_cases` still pass `None` and keep `root/target`.
- **Target isolation.** The target is `CARGO_TARGET_TMPDIR/composite-equality-agreement-target`,
  per worktree unless a caller shares `CARGO_TARGET_DIR`; that sharing already raced on the
  scratch roots before this PR. No corpus or label collides with this directory. `tc_029` runs
  its cases serially, so nothing contends for the shared target inside one run.
- **Scope of change.** No `.github`, `spec`, `Makefile`, `Cargo.toml` or `Cargo.lock` change.

Gates run by this reviewer at the reviewed head: `cargo fmt --all --check` and
`git diff --check` pass. Clippy and the tests were not rerun: the worktree has no `target/`, and
the brief reserves the aggregate run for the code lead. The author's logs show clippy completing
and `tc_029` passing (1 passed, 228.19s, wall 228.43s), written after the head commit.

Measurement limits:

- 228.19s against 2659.80s and 2816.84s are all genuine log values for the focused test and are
  about 11.7 to 12.3 times apart, but they span different commits and host load and are not a
  controlled comparison.
- The cold-scratch-target claim and the 15-directory observation are not in the log, and the
  worktree's `target/` is gone, so neither could be independently confirmed.
- `cg-ir1116-isolation.log` is empty, so the isolation test has no retained evidence.
- The figure excludes the outer `it` build and says nothing about the aggregate gate. CI runner
  times are unmeasured; a warm rerun should be cheaper, but that too is unmeasured.

quoin gap-analysis did not run: the diff touches only `tests/`, so the reviewer rule for
production code outside `spec/` and `plan/` does not apply. A manual check of the ticket's
acceptance text found FND-003.

## Findings

| ID      | Severity | Summary                                                                                                                                          | Refs                                  |
| ------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------- |
| FND-001 | high     | Generated package name is hand-typed in the test instead of using the exported COMPOSITE_EQUALITY_CRATE_NAME                                     | tests/it/exact_scalar_agreement.rs:175 |
| FND-002 | medium   | New harness test is trace-bound to FR-018-AC-2 but asserts no oracle, runtime charge or counter agreement                                        | tests/it/exact_scalar_agreement.rs:194 |
| FND-003 | medium   | Ticket's time attribution and aggregate before/after are unrecorded while other corpora keep cold per-crate dependency builds                    | tests/it/exact_scalar_agreement.rs:102 |
| FND-004 | low      | Isolation regression calls the rename helper directly, so runner wiring and per-case name uniqueness stay untested                               | tests/it/exact_scalar_agreement.rs:209 |
| FND-005 | low      | Module doc still names run_agreement_cases as the runner composite equality shares, and the new doc omits that the label becomes the package name | tests/it/exact_scalar_agreement.rs:9  |

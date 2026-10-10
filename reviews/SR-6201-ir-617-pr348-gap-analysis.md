---
id: SR-6201
title: "gap-analysis review of IR-617 PR 348"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@2f5f078a3a141f4a13e0dc1f1df3e428ff7f1446; Makefile, scripts/kani_gate.sh, scripts/kani_scope.sh, tests/it/kani_gate.rs, tests/it/main.rs, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md"
review_set: subset
---

## Summary

Computed NFR-006 matrix binds AC-1–12, AC-14 and AC-18; AC-13/15/16/17 are inspection criteria. AC-7 binding is semantically incomplete. Ticket: IR-617.

## Verdict

**CONDITIONAL** — Computed NFR-006 matrix binds AC-1–12, AC-14 and AC-18; AC-13/15/16/17 are inspection criteria. AC-7 binding is semantically incomplete.

## Examined scope

- `NFR-006-AC-1` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): `make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`, `src/oracle/boolean_v1.rs`, `src/routed/generate.rs`, `src/replay/witness.rs`, `src/core/identity.rs`, `src/publication/mod.rs`, `tests/it/kani_batching.rs`, `tests/it/skeleton_spine.rs`, `tests/it/bounded_kani_corpus.rs`, `tests/it/kani_obligations_state_frame.rs`, `tests/it/scratch_crate.rs`, `tests/exact_scalar_support/package.rs`, `tests/checked_package_support/base.rs`, `tests/checked_package_support/rekey.rs`, `tests/state_frame_support/model.rs`, `schemas/kani-proof-graph-v2.schema.json`, `sch
- `NFR-006-AC-2` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): `make kani-scope` prints `not required` for each of `spec/kani/functional/FR-017-kani-execution-evidence.md`, `reviews/REV-018-bound-coverage-observations.md`, `src/strategy/mod.rs`, `src/evidence/mod.rs`, `Makefile` and `.github/workflows/ci.yml`, each fed alone.
- `NFR-006-AC-3` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): A rename of `src/kani/old.rs` to `src/strategy/new.rs`, and a rename of `src/strategy/old.rs` to `src/kani/new.rs`, each print `required`.
- `NFR-006-AC-4` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): `make -n kani` and `make -n kani-gate` expand to the same `cargo test` command line, and that line holds each of the six filters `kani_obligations`, `skeleton_spine`, `kani_witness_join`, `bounded_kani_corpus`, `kani_generation` and `kani_batching`.
- `NFR-006-AC-5` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): `make kani-gate` starts the lane's `cargo test` once with `--test-threads=1`, holds the lock named by `KANI_LOCK` while it runs, and does not start it while another process holds that lock.
- `NFR-006-AC-6` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): `make kani-gate` exits non-zero and prints `result=failed` when the lane's filters list zero tests, when fewer tests run than are listed, and when any test fails; it exits zero and prints `result=passed` only when every listed test ran and passed and the count is above zero.
- `NFR-006-AC-7` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): The passing run-result line has the fields `result`, `ran`, `expected`, `elapsed`, `kani`, `tree` in that order, with `ran` equal to `expected`, a non-empty `kani` equal to the version the launcher reports, and `tree=clean`; it has no commit token. A run whose launcher reports no version prints `result=failed`.
- `NFR-006-AC-8` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): When the tree state or the head commit read after the run differs from the one read before the build, `make kani-gate` prints `result=failed` and exits non-zero.
- `NFR-006-AC-9` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): With a dirty working tree `make kani-gate` prints `tree=dirty`, never `result=passed`, and exits non-zero.
- `NFR-006-AC-10` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): With no `cargo-kani` launcher `make kani-gate` starts no test, prints `kani-gate: not run: launcher absent` and exits non-zero.
- `NFR-006-AC-11` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): The tests marked `#[ignore = "kani lane: ..."]` in `tests/it` are exactly the `#[ignore]`d tests of `tests/it` whose path holds one of the `make kani` filters.
- `NFR-006-AC-12` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): `make -n ci` expands to no `kani` or `kani-gate` command.
- `NFR-006-AC-14` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): Given Kani-reaching paths changed on `origin/main` since the candidate's merge base and main absent from the candidate, `make kani-gate` refuses before running tests and names those paths. Given only non-touching main changes, it may run. The merger updates a stale candidate from current main and the full gate passes on the updated clean candidate before merge.
- `NFR-006-AC-18` (examined, `spec/kani/non-functional/NFR-006-real-kani-lane-gating.md`): Every `include!`, `include_str!`, `include_bytes!` and `#[path]` target named in a file of `tests/it` that holds a lane test, and in each file those name in turn, matches a pattern of the Kani-touching set.

## Coverage

Plan completion: not assessed. Computed Test Matrix checked for NFR-006 only; repository-wide strict matrix has unrelated pre-existing gaps. Intent-to-test agreement was inspected for AC-7 as requested in this scoped review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The computed matrix marks AC-7 tagged, but its only tagged test does not assert elapsed or the required field order; the criterion is only partly verified. | tests/it/kani_gate.rs:231-235 |

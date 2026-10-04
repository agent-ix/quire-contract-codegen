---
id: TC-045
title: "Verify the real-Kani lane gate targets"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: verifies
---
# TC-045: Verify the real-Kani lane gate targets

## Description

Verify that `make kani-scope` decides from changed paths whether the real-Kani lane is required,
that `make kani-gate` runs the lane serially under the host lock and reports an evidence line that
cannot read as a pass when nothing ran, and that the lane selects every real-Kani test (NFR-006).
The pull-request criteria are inspection, below.

## Test Procedure

The targets call `scripts/kani_scope.sh` and `scripts/kani_gate.sh` (planned), so a test can hand
them stand-in executables on `PATH` for `cargo`, `cargo-kani` and `git` without installing Kani. The
test is `tests/it/kani_gate.rs` (planned) and runs in the default `cargo test`.

1. Scope, required (NFR-006-AC-1): feed `src/kani/generate/negotiate.rs`, `src/oracle/boolean_v1.rs`,
   `src/routed/generate.rs`, `src/replay/witness.rs`, `src/core/identity.rs`,
   `tests/it/kani_batching.rs` and `Cargo.lock` as changed paths, each alone, and assert `required`
   and the path on the output.
2. Scope, not required (NFR-006-AC-2): feed `spec/kani/functional/FR-017-kani-execution-evidence.md`,
   `reviews/REV-018-bound-coverage-observations.md`, `src/strategy/mod.rs`, `Makefile` and
   `.github/workflows/ci.yml`, each alone, and assert `not required`.
3. Lock and serial (NFR-006-AC-3): with a stand-in `cargo` that records its argument vector and the
   moment it started, hold `/tmp/agent-e-heavy-build.lock` from another process for a fixed interval,
   start the gate, and assert the stand-in did not start until the lock was released, was started
   once, and was passed `--test-threads=1`.
4. Counts (NFR-006-AC-4): with the stand-in listing 27 tests, reporting 27 passed: `result=passed`,
   exit 0. Reporting 26 passed: `result=failed`, non-zero. Listing 0 and reporting 0 passed:
   `result=failed expected=0`, non-zero. Listing 27 and reporting one failure: `result=failed`,
   non-zero.
5. Line (NFR-006-AC-5): on the passing run in step 4, assert the line matches the field order
   `result`, `ran`, `expected`, `elapsed`, `kani`, `tree`, `head`, with `ran` equal to `expected`,
   `tree=clean` and `head` equal to the stand-in `git`'s head.
6. Dirty tree (NFR-006-AC-6): with the stand-in `git` reporting a dirty tree and the tests passing,
   assert `tree=dirty`, no `result=passed` anywhere on the output, and a non-zero exit.
7. Launcher absent (NFR-006-AC-7): with no `cargo-kani` on `PATH`, assert the stand-in `cargo` is
   never started, the line is `kani-gate: not run: launcher absent` and the exit is non-zero.
8. Selection (NFR-006-AC-8): collect the names of the tests carrying `#[ignore = "kani lane` by a
   `syn` walk of `tests/it`, run `cargo test --test it -- --ignored --list` with the `make kani`
   filters, and assert the two sets are equal; a test tagged `kani lane` that no filter selects, or a
   selected test without the tag, fails the step.
9. `make ci` (NFR-006-AC-9): run `make -n ci` and assert no line contains `kani`.
10. Inspection (NFR-006-AC-10 to NFR-006-AC-12): for each merged pull request that touched the
    Kani-touching set, read its body for the evidence line and compare `head` with the pull request's
    last commit; for each such pull request that ran no lane, read its body for
    `kani-gate: not run: <reason>` and for the absence of a claim that the lane verified the
    change; for each pushed version tag, read the release ticket for its line.

## Expected Results

The scope target is a pure function of the changed paths. The gate prints `passed` only for a run in
which every listed test ran and passed on a clean tree, serially, under the lock, and prints a
`not run` or `failed` line otherwise, with a non-zero exit. The lane selects exactly the tests that
carry its tag. `make ci` does not run the lane.

## Implementation

Planned: `scripts/kani_scope.sh`, `scripts/kani_gate.sh`, the `kani-scope` and `kani-gate` targets in
`Makefile`, and `tests/it/kani_gate.rs`. Steps 1 to 9 are tests; step 10 is inspection. No CI
workflow is part of this test case.

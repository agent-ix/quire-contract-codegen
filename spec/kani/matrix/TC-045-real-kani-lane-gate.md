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
that `make kani-gate` runs the same recipe as `make kani` serially under the host lock and reports a
run result that cannot read as a pass when nothing ran or the tree is dirty, and that the lane selects
every real-Kani test (NFR-006). The pull-request and release criteria are inspection, step 13.

## Test Procedure

The targets call the `cg_gates` Rust binary, and the Makefile
defines the `kani` recipe once, in a variable the gate calls. A test can hand them stand-in
executables on `PATH` for `cargo`, `cargo-kani` and `git`, and a temporary lock file through
`KANI_LOCK`, without installing Kani and without touching `/tmp/agent-e-heavy-build.lock`. The test is
`tests/it/kani_gate.rs` and runs in the default `cargo test`. It starts no nested `cargo
test` and holds no host-wide lock.

1. Scope, required (NFR-006-AC-1): feed each of the nineteen paths of NFR-006-AC-1 alone as the
   changed paths and assert `required` and the path on the output.
2. Scope, not required (NFR-006-AC-2): feed each of the six paths of NFR-006-AC-2 alone and assert
   `not required`.
3. Rename (NFR-006-AC-3): supply both `src/kani/old.rs` and `src/strategy/new.rs`, then
   both `src/strategy/old.rs` and `src/kani/new.rs`; assert `required` for each input pair.
4. One recipe (NFR-006-AC-4): run `make -n kani` and `make -n kani-gate`; assert the same `cargo test`
   command line in both and that it holds each of the eight current filters, `--ignored` and
   `--test-threads=1`.
5. Lock and serial (NFR-006-AC-5): set `KANI_LOCK` to a temporary file. The stand-in `cargo` writes a
   marker file when it starts and records whether `flock -n` on `KANI_LOCK` fails (the gate holds it).
   Assert one start, the `--ignored` and `--test-threads=1` arguments, and a failed `flock -n`.
   Step 4 checks that the Make caller passes all eight filters. Then hold the temporary lock from
   the test, start the gate, wait until a process whose command line names `flock` and the temporary
   lock path appears in the process table, assert the marker file is absent, release the lock and
   wait for the marker file. No wall-clock interval orders the steps.
6. Counts (NFR-006-AC-6): with a stand-in, listing zero tests gives `result=failed expected=0`
   and no test start; listing two but reporting one passed, or reporting a test failure, gives
   `result=failed` and non-zero. A stand-in that reports all listed tests passed also gives
   `result=failed`: only the trusted Cargo command can discharge the proof. Step 13 observes the
   positive result with the installed Kani lane.
7. Run result (NFR-006-AC-7): on a stand-in run that lists and reports the same nonzero count,
   assert the field order `result`, `ran`, `expected`, `elapsed`, `kani`, `tree`, with no commit token;
   assert `ran` equals `expected`, `kani` equals the stand-in `cargo-kani --version` output, and
   `tree=clean`, while `result=failed` and exit non-zero preserve the trusted-Cargo refusal. With
   a stand-in `cargo-kani` that prints no version, assert `result=failed` and non-zero. Step 13
   checks the `result=passed` line from a successful trusted installed-Kani run.
8. Post-run tree (NFR-006-AC-8): with a stateful stand-in `git` that reports a clean
   tree before the build and a dirty tree after the run, assert `tree=dirty`, no
   `result=passed`, and a non-zero exit. Assert that the gate reads no Git revision.
   NFR-006-AC-14's merge-base comparison is retired.
9. Dirty tree (NFR-006-AC-9): with the stand-in `git` reporting a dirty tree before the build and the
   tests passing, assert `tree=dirty`, no `result=passed` on the output, and a non-zero exit.
10. Launcher absent (NFR-006-AC-10): with no `cargo-kani` on `PATH`, assert the stand-in `cargo` never
    started, the line is `kani-gate: not run: launcher absent` and the exit is non-zero.
11. Selection (NFR-006-AC-11), with no nested cargo: a `syn` walk of `tests/it` collects the module
    path and function name of every `#[ignore]`d test and notes which carry `#[ignore = "kani lane`.
    Confirm the eight named filters in `make -n kani`. Apply libtest's rule (a filter selects a test whose full
    path contains it) and assert that the tagged tests and the `#[ignore]`d tests the filters select are
    the same set; a tagged test no filter selects, or a selected test without the tag, fails the step.
    Include closure (NFR-006-AC-18): in the same walk, read every `include!`, `include_str!`,
    `include_bytes!` and `#[path]` target from each `tests/it` file that holds a lane test, and from
    each file those name in turn, resolve it against the naming file's directory, and assert the
    result matches the Kani-touching set implemented by `cg_gates kani-scope`; the targets
    `tests/checked_package_support/base.rs`, `tests/checked_package_support/rekey.rs`,
    `tests/state_frame_support/subject.rs` and the three Kani
    schemas must be found.
12. `make ci` (NFR-006-AC-12): run `make -n ci` and assert no line contains `kani`.
13. Inspection (NFR-006-AC-13, AC-15 to AC-17): for a Kani-touching pull request, observe the
    author run the actual full gate on the clean, stable candidate before opening it. Before merge,
    check that the current candidate contains the pull request's changes. Observe a fresh full gate on
    that clean, stable candidate immediately before merge; require successful process exit and every
    listed test run. A copied PR-body status line does not pass this step. A failed, absent or skipped
    gate leaves the pull request unmerged and without a verification claim. For a `not required`
    scope, no real-Kani run is mandatory. Before a version tag is pushed, observe the actual full
    gate pass on the clean, stable candidate to be tagged; a release-ticket line is insufficient.

## Expected Results

The scope target is a pure function of the changed paths, renames included. The gate prints `passed`
only for a run in which every listed test ran and passed on a clean tree, serially,
under the lock, with the `make kani` command line, and prints a `not run` or `failed` line otherwise,
with a non-zero exit. The lane selects exactly the tests that carry its tag. `make ci` does not run
the lane. Before a Kani-touching pull request opens or merges, and before a tag is pushed, the actual
gate passes on its current clean candidate; a copied status line is insufficient.

## Implementation

Implemented: `scripts/cg_gates.rs`, the `kani-scope` and `kani-gate`
targets and the shared `kani` recipe variable in `Makefile`, and `tests/it/kani_gate.rs`.
Steps 1 to 12 are tests; step 13 is inspection. No CI workflow is part of this test case.

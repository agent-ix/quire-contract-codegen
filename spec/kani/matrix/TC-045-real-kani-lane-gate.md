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
that `make kani-gate` runs the same recipe as `make kani` serially under the host lock and reports an
evidence line that cannot read as a pass when nothing ran or the tree moved, and that the lane selects
every real-Kani test (NFR-006). The pull-request criteria are inspection, step 11.

## Test Procedure

The targets call `scripts/kani_scope.sh` and `scripts/kani_gate.sh` (planned), and the Makefile
defines the `kani` recipe once, in a variable the gate calls. A test can hand them stand-in
executables on `PATH` for `cargo`, `cargo-kani` and `git`, and a temporary lock file through
`KANI_LOCK`, without installing Kani and without touching `/tmp/agent-e-heavy-build.lock`. The test is
`tests/it/kani_gate.rs` (planned) and runs in the default `cargo test`. It starts no nested `cargo
test` and holds no host-wide lock.

1. Scope, required (NFR-006-AC-1): feed each of the eighteen paths of NFR-006-AC-1 alone as the
   changed paths and assert `required` and the path on the output.
2. Scope, not required (NFR-006-AC-2): feed each of the six paths of NFR-006-AC-2 alone and assert
   `not required`.
3. Rename (NFR-006-AC-3): have the stand-in `git`, asked for names with rename detection off, report
   `src/kani/old.rs` and `src/strategy/new.rs`, then `src/strategy/old.rs` and `src/kani/new.rs`;
   assert `required` for each. Assert the script's `git` argument vector turns rename detection off.
4. One recipe (NFR-006-AC-4): run `make -n kani` and `make -n kani-gate`; assert the same `cargo test`
   command line in both and that it holds each of the six filters, `--ignored` and
   `--test-threads=1`.
5. Lock and serial (NFR-006-AC-5): set `KANI_LOCK` to a temporary file. The stand-in `cargo` writes a
   marker file when it starts and records whether `flock -n` on `KANI_LOCK` fails (the gate holds it).
   Assert one start, an argument list equal to the `cargo test` line of step 4 (all six filters,
   `--ignored`, `--test-threads=1`) and a failed `flock -n`. Then hold the temporary lock from
   the test, start the gate, wait until a process whose command line names `flock` and the temporary
   lock path appears in the process table, assert the marker file is absent, release the lock and
   wait for the marker file. No wall-clock interval orders the steps.
6. Counts (NFR-006-AC-6): the stand-in listing 27 tests and reporting 27 passed gives `result=passed`
   and exit 0; reporting 26 passed gives `result=failed` and non-zero; listing 0 and reporting 0 gives
   `result=failed expected=0` and non-zero; listing 27 and reporting one failure gives
   `result=failed` and non-zero.
7. Line (NFR-006-AC-7): on the passing run, assert the field order `result`, `ran`, `expected`,
   `elapsed`, `kani`, `tree`, `head`, `ran` equal to `expected`, `kani` equal to the version the
   stand-in `cargo-kani --version` prints, `tree=clean` and `head` equal to the stand-in `git`'s head.
   With a stand-in `cargo-kani` that prints no version assert `result=failed` and non-zero.
8. Re-read (NFR-006-AC-8): with a stateful stand-in `git` whose head is `A` before the build and `B`
   after, and one whose tree is clean before and dirty after, assert `result=failed` and non-zero
   for each.
9. Dirty tree (NFR-006-AC-9): with the stand-in `git` reporting a dirty tree before the build and the
   tests passing, assert `tree=dirty`, no `result=passed` on the output, and a non-zero exit.
10. Launcher absent (NFR-006-AC-10): with no `cargo-kani` on `PATH`, assert the stand-in `cargo` never
    started, the line is `kani-gate: not run: launcher absent` and the exit is non-zero.
11. Selection (NFR-006-AC-11), with no nested cargo: a `syn` walk of `tests/it` collects the module
    path and function name of every `#[ignore]`d test and notes which carry `#[ignore = "kani lane`.
    Read the six filters from `make -n kani`. Apply libtest's rule (a filter selects a test whose full
    path contains it) and assert that the tagged tests and the `#[ignore]`d tests the filters select are
    the same set; a tagged test no filter selects, or a selected test without the tag, fails the step.
    Include closure (NFR-006-AC-18): in the same walk, read every `include!`, `include_str!`,
    `include_bytes!` and `#[path]` target from each `tests/it` file that holds a lane test, and from
    each file those name in turn, resolve it against the naming file's directory, and assert the
    result matches a pattern of the Kani-touching set read from `scripts/kani_scope.sh`; the targets
    `tests/checked_package_support/base.rs`, `tests/state_frame_support/subject.rs` and the three Kani
    schemas must be found.
12. `make ci` (NFR-006-AC-12): run `make -n ci` and assert no line contains `kani`.
13. Inspection (NFR-006-AC-13 to NFR-006-AC-17): for each merged pull request that touched the
    Kani-touching set, read its body for the evidence line and compare `head` with the pull request's
    last commit, and list the commits of `origin/main` that touched the set at the merge and check
    the head contains each; for each such pull request that ran no lane, read its body for
    `kani-gate: not run: <reason>`, for the absence of a claim that the lane verified the change, and
    that it is unmerged; for each pull request whose scope was `not required`, read its body for
    `kani-gate: not required`; for each pushed version tag, read the release ticket for its line.

## Expected Results

The scope target is a pure function of the changed paths, renames included. The gate prints `passed`
only for a run in which every listed test ran and passed on a clean tree that did not move, serially,
under the lock, with the `make kani` command line, and prints a `not run` or `failed` line otherwise,
with a non-zero exit. The lane selects exactly the tests that carry its tag. `make ci` does not run
the lane.

## Implementation

Planned: `scripts/kani_scope.sh`, `scripts/kani_gate.sh`, the `kani-scope` and `kani-gate` targets and
the shared `kani` recipe variable in `Makefile`, and `tests/it/kani_gate.rs`. Steps 1 to 12 are tests;
step 13 is inspection. No CI workflow is part of this test case.

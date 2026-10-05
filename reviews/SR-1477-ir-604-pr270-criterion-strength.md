---
id: "SR-1477"
title: "CG PR 270 spec review (criterion strength): NFR-006 ACs and TC-045 steps"
type: SpecReview
analysis: criterion-strength
review_set: subset
date: "2026-10-04"
scope: "agent-ix/quire-contract-codegen@3da228d9eccae9196372574bd16e58a1ef5264f6; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md (AC-1 to AC-12), spec/kani/matrix/TC-045-real-kani-lane-gate.md (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-045
    type: reviews
---

# SR-1477: CG PR 270 spec review (criterion strength)

## Summary

Ticket: IR-604. Each NFR-006 AC was judged on whether a wrong gate could still pass it, with the
TC-045 step that drives it.

What holds: AC-4 covers the empty-filter case (libtest exits 0 when no test matches), fewer-ran and a
failure, and passes only on `ran == expected > 0`. AC-6 covers a dirty tree and AC-7 a missing
`cargo-kani`. The merger's head comparison catches a run on another commit. The lane tests need a
real installation (`KaniInstallation::discover().expect(...)`, with no early return), so a test cannot
pass with Kani absent. AC-8 catches a filter dropped from `make kani` or a module unregistered from
`tests/it/main.rs`. AC-9 holds today: `make -n ci` prints no `kani` line. The stand-ins on `PATH`
make steps 1 to 9 testable without Kani.

## Verdict

Changes requested. Three medium, one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Nothing ties `make kani-gate`'s selection or command to `make kani`. AC-3 asserts only `--test-threads=1`, one process and the lock. AC-4's `expected` comes from the gate's own filters. AC-8 compares the tag set with `make kani`'s filters, not the gate's. A `scripts/kani_gate.sh` with a stale or shorter filter list, or without `+1.98.1`, `--locked` or the target dir, passes AC-3, AC-4 and AC-8 and prints `result=passed` over a subset. Add an AC that the gate runs exactly `make kani`'s argument vector (it calls the target, or both read one filter variable), and have TC-045 step 3 assert the full filter list. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:129-130, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:134, spec/kani/matrix/TC-045-real-kani-lane-gate.md:31-34 |
| FND-002 | medium | The spec does not say when `tree` and `head` are measured. A lane run lasts tens of minutes on a host shared with agents. A tree clean at the start can be edited, or HEAD moved, before cargo compiles, and the line still prints `tree=clean head=<start>` over different content. Require both to be read before the build and re-read after the run, and print `result=failed` if either changed. Add a TC-045 case where the stand-in `git` changes between the two reads. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:32, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:131-132 |
| FND-003 | medium | Two TC-045 steps are unsafe to run in the default `cargo test`, which `make ci` runs twice per change. (a) Step 3 holds the real host lock `/tmp/agent-e-heavy-build.lock`, which the spec fixes as the path. On the shared host the test blocks, or is blocked by, a real heavy build such as a lane run of more than 1000 s. It also depends on a "fixed interval" of wall-clock time, and #266 only just fixed a load flake of that kind. Make the lock path an override the test sets, and order the steps with a marker file, not a delay. (b) Step 8 calls `cargo test --test it -- --ignored --list` from inside a running `cargo test`. On the same target dir that waits on cargo's build lock. On another target dir it rebuilds the `it` binary at MSRV inside the default suite. Specify the comparison without nested cargo: apply libtest's substring rule for the filters to the names collected by `syn`. Or move step 8 into the lane. | spec/kani/matrix/TC-045-real-kani-lane-gate.md:31-34, spec/kani/matrix/TC-045-real-kani-lane-gate.md:46-49 |
| FND-004 | low | Three ACs are weaker than they look. AC-1 samples the set but has no case for `tests/it/skeleton_spine.rs`, `tests/it/bounded_kani_corpus.rs` or `tests/it/kani_obligations_state_frame.rs` (only the glob covers it). No AC covers a rename: `git diff --name-only` reports only the destination, so a file moved out of `src/kani/` reports only its new path. AC-5 checks field order but not that `kani=` holds the version the launcher reports, so an empty `kani=` passes. AC-2 says "a change touching only" the five paths, but TC-045 feeds each one alone. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:127-128, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:131 |

## New findings (disposition pass 1)

Found in round 1, reviewed at d4e8ed1de90cd8bd420bf87d1469a109d198ca76.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | AC-4 ties the two targets only as text under `make -n`. TC-045 step 5 records the stand-in `cargo`'s argument vector but asserts only one start and `--test-threads=1`. So `scripts/kani_gate.sh` could receive the shared recipe line, run a different `cargo test`, and still pass AC-4 and AC-5. Have step 5 assert that the vector the stand-in received equals the AC-4 line, from `test` through the six filters. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:155-156, spec/kani/matrix/TC-045-real-kani-lane-gate.md:37-42 |

## Dispositions

Round 1, reviewed at d4e8ed1de90cd8bd420bf87d1469a109d198ca76 (fix commit d4e8ed1 on top of 3da228d).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d4e8ed1: a statement says `make kani-gate` runs the `make kani` recipe once, with one definition for toolchain, flags, target dir and filters. AC-4 says `make -n kani` and `make -n kani-gate` expand to the same `cargo test` line, holding all six filters, and TC-045 step 4 checks it. A residual at the stand-in level is FND-005. |
| FND-002 | fixed | d4e8ed1: a statement requires tree and head to be read before the build and after the run, printing `result=failed` on any difference (lines 38-39). AC-8 states it, and TC-045 step 8 uses a stateful stand-in `git` (head A then B; clean then dirty). |
| FND-003 | fixed | d4e8ed1: the lock is `KANI_LOCK`, defaulting to the host path. TC-045 says the test holds no host-wide lock and starts no nested `cargo test`. Step 5 uses a temporary lock, waits on the `flock` process and a marker file, and uses no wall-clock interval. Step 11 is a `syn` walk plus the filters read from `make -n kani`, with libtest's substring rule applied, so no cargo runs. |
| FND-004 | fixed | d4e8ed1: AC-1 lists 15 paths, each fed alone, covering spine, corpus, state_frame, publication, scratch_crate, both support paths, Cargo.toml and Cargo.lock. AC-3 checks a rename in each direction, with rename detection off (statement lines 24-25; step 3 asserts the argument). AC-7 requires a non-empty `kani` equal to the launcher's version, and `result=failed` when no version is reported. AC-2 says "each fed alone". |

Round 2, reviewed at 1c26606b6e97ddc8795a19e5d3cdb04ed4e2fba4 (fix commit 1c26606 on top of d4e8ed1).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 1c26606: TC-045 step 5 now asserts "an argument list equal to the `cargo test` line of step 4 (all six filters, `--ignored`, `--test-threads=1`)" for the stand-in `cargo`, so a gate script that runs a different `cargo test` fails. |

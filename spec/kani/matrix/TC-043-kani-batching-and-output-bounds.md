---
id: TC-043
title: "Verify Kani harness batching, the output cap and launcher cleanup"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: verifies
---
# TC-043: Verify Kani harness batching, the output cap and launcher cleanup

## Description

Verify that several harnesses run in one launcher process with each member decided from its own
report entry and its own console playback, that an output stream over its limit is refused and never
truncated, that a capture failure is refused, and that no launcher descendant outlives a run
(IR-277; FR-017-AC-14 and FR-017-AC-21 to FR-017-AC-25). The batch timeout rule it exercises is
FR-028-AC-12, verified by TC-039.

## Test Procedure

1. Drive a launcher stand-in that prints more than 8 MiB on stdout, then on stderr; one that prints
   exactly 8 MiB; and a batch stand-in of N members that prints more than N times 8 MiB.
2. Drive a capture whose reader thread panics, and one whose pipe read returns an error.
3. Run a launcher that exits on its own, leaving a real grandchild in its process group.
4. Build N = 1, 10 and 50 harnesses with equal option vectors and request timeout T, and read the
   launch's argument vector and the number of launcher processes a stand-in counts; mix two option
   vectors and two timeouts and count the processes.
5. Build a batch whose T is `Duration::MAX` and read its argument vector; in the `make kani` lane
   run a batch at 4294967295 seconds and confirm it runs.
6. Run batch stand-ins that exit successfully with no report, and unsuccessfully with no report (an
   argument error, a failed build).
7. Feed batch reports (captured from real Kani 0.68 in the `make kani` lane, and synthetic): one
   member falsified beside a verified one; two members sharing the bare symbol `check` under
   different `module::harness` paths; every member Success with a non-zero exit; one Failure entry
   with a non-zero exit; a Failure entry with no checks and exit status `timeout`; one with no checks
   and no timeout; a report that lacks, duplicates or adds a requested harness.
8. Feed a console in which an earlier member prints a failing playback block and a later falsified
   member prints its own; one in which a falsified member has no block headed for its path; one with
   a block headed for a path that is not a member; and one with two blocks for one path.
9. Leave one member's source out of the crate.
10. In the `make kani` lane record the process count and wall time before and after batching for
    N = 1, 10 and 50.

## Expected Results

1. Each over-limit stream is refused with `kani_output_over_limit` naming the stream, the limit and
   the harness count, the group is killed, and nothing is classified; exactly the limit completes
   with its real exit status and the whole stream (FR-017-AC-14).
2. Both are refused with `kani_output_unread` and no outcome (FR-017-AC-25).
3. The grandchild is killed by the time the run returns (FR-017-AC-24).
4. One process for each group, with one `--harness <module::harness> --exact` pair per member in
   request order, `--harness-timeout` T and then the shared options; fewer than N processes for
   N > 1 compatible harnesses; the outer bound is N times T and a batch killed at it is refused as
   timed out with no member classified (FR-017-AC-21).
5. The `Duration::MAX` batch carries no `--harness-timeout` and its outer bound does not elapse; the
   batch at 4294967295 seconds runs (FR-017-AC-21, FR-028-AC-12).
6. The successful exit with no report is refused with the missing-report refusal and no member is
   classified; the unsuccessful exit with no report leaves every member inconclusive `NoVerdict`
   (FR-017-AC-21).
7. Members are matched by `module::harness`; a falsified member beside a verified one leaves the
   verified one verified; Success in a non-zero exit with no Failure entry is inconclusive and with
   one is verified; the timeout entry is inconclusive timed-out naming T while the others keep their
   results; the entry with no checks and no timeout is inconclusive with no counterexample; each
   member's evidence carries its kind, harness path, launcher path, unwind bound, solver, outcome and
   checks plus the batch vector, the member list, the batch statement and the exit code
   (FR-017-AC-22). A missing, duplicated or unrequested harness refuses the whole batch
   (FR-017-AC-23).
8. Each falsified member's playback is the block headed for its own path, never an earlier member's;
   the falsified member with no block is inconclusive with no counterexample and its neighbours keep
   their results; the block for a non-member path and the pair of blocks for one path each refuse
   the whole batch (FR-017-AC-23).
9. The batch is refused with `HarnessNotInCrate` and launches nothing (FR-017-AC-23).
10. Fewer processes after than before for N > 1, and the numbers are in the code PR.

## Status

Planned (IR-277). At this revision the launcher runs one harness per process, keeps the tail of an
over-long stream silently, falls back to empty text when a capture thread panics and kills the group
only on a timeout.

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
4. Build N = 1, 10 and 50 harnesses with equal option vectors and identity `ProofCeilings`
   (`wall_clock` T and `memory_bytes` M), and read the launch's argument vector and the number of
   launcher processes a stand-in counts; mix two option vectors, two wall-clock ceilings and two
   memory ceilings and count the processes. Drive one grouped process tree above M.
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
   a block headed for a path that is not a member; and the console of a real run in which one
   member fails two property checks and so has two counterexample blocks under its path.
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
   N > 1 compatible harnesses; differing identity wall-clock or memory ceilings, launcher, crate
   directory or target directory form separate groups. The grouped process tree has one aggregate
   M ceiling; exceeding it kills and refuses the whole group with no member classified. The outer
   wall-clock bound is N times T (a group that runs longer than T and inside N times T completes),
   and a group killed at it is refused as timed out with no member classified (FR-017-AC-21;
   FR-028-AC-21 covers the memory enforcement mechanism).
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
   checks plus the group's vector, the member list, the batch statement and the exit code
   (FR-017-AC-22); a sub-second T is carried rounded up. A missing, duplicated or unrequested
   harness refuses the group (FR-017-AC-23).
8. Each falsified member's playback is the block headed for its own path, never an earlier member's;
   the falsified member with no block is inconclusive with no counterexample and its neighbours keep
   their results; the block for a non-member path refuses the group; the member with two
   counterexample blocks takes the first, as the same harness alone does, and neither it nor its
   neighbour loses its evidence (FR-017-AC-23).
9. The whole batch is refused with `HarnessNotInCrate` and launches nothing (FR-017-AC-23).
10. Fewer processes after than before for N > 1, and the numbers are in the code PR (they are also
    written by the lane to `process-counts.json`).

## Implementation

Launcher stand-ins and real short-lived processes, in `src/kani/run/launch.rs`:
`tc_043_a_capture_over_its_limit_is_refused_and_one_at_its_limit_is_whole`,
`tc_043_a_launcher_stream_over_the_limit_is_refused_and_one_at_the_limit_completes`,
`tc_043_a_batch_stream_is_bounded_by_the_limit_times_the_member_count` and
`tc_043_an_over_limit_run_is_stopped_and_kills_the_launcher_group` (step 1; FR-017-AC-14),
`tc_043_a_capture_that_panics_or_whose_poll_or_read_errs_is_unread_not_empty` and
`tc_043_an_unread_stream_names_the_stream_in_the_launch_outcome` (step 2; FR-017-AC-25), and
`tc_043_a_launcher_that_exits_on_its_own_has_its_grandchild_killed` (step 3; FR-017-AC-24).

A shell-script launcher that records each process it is started as and the arguments it received,
in `src/kani/run/execute.rs` (`batch_tests`): the process counts and argument vectors for N = 1, 10
and 50 compatible harnesses and for mixed option vectors and identity wall-clock ceilings, the
`--harness-timeout` and outer-bound arithmetic including `Duration::MAX` and 4294967295 seconds, a
batch killed at its outer
bound, both no-report exits, the keyed split of a report whose results are in Kani's own order
rather than the request's (two members sharing the bare symbol `check`), the exit-status
predicate, the `timeout` entry (a sub-second T rounded up), the lacking, repeating and unrequested
harness, the member missing from the crate, the grouping of requests that name another launcher,
crate directory or target directory, distinct identity memory ceilings, the batch that runs longer
than T and inside N times T, the playback attribution cases (including
`tc_043_a_member_failing_two_checks_takes_its_first_block_and_keeps_its_neighbour`,
whose console is the playback section of a real Kani 0.68 batch, verbatim) and the over-limit
refusal of a group by its member count (steps 4 to 9).
`tc_043_a_playback_is_taken_by_the_path_it_is_headed_for` in `src/kani/output/playback.rs` covers
the path-keyed scan.

The `make kani` lane runs real Kani 0.68 in `tests/it/kani_batching.rs`: the launcher process count
and wall time before and after batching for N = 1, 10 and 50 (step 10), and one real batch of a
verified, a falsified, a two-failed-assertion and a timed-out member (T = 20 s, after a warm-up
build so that the outer bound never covers a cold compile), then one at 4294967295 seconds (steps 5
and 7). The unit lane's batch reports are built in the shape the real batch report has
(`harness_id`, an `error_details` entry whose `exit_status` is `timeout` for a member Kani cut off,
results in the order Kani ran them); no real batch report is stored as a fixture.

## Status

Implemented for FR-017 batching and output bounds (IR-277). Identity memory-ceiling separation is
covered by `unequal_identity_memory_ceilings_run_in_separate_backend_processes` and
`unequal_memory_ceilings_plan_separate_groups_with_their_own_selections` in
`src/kani/run/execute.rs`; grouped aggregate memory exhaustion is covered under TC-039
(FR-028-AC-21).

---
id: "SR-1465"
title: "PR 266 code review (incl. rust-review lane): batch outer-bound and orphan timing tests"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@0a238efb9b07399517a67585806356a58e466b1a; src/kani/run/execute.rs, src/kani/run/launch.rs"
relationships: []
---

# SR-1465: PR 266 code review

## Summary

Ticket: none (the PR fixes a flake in a test that merged #259, IR-277, added). PR:
agent-ix/quire-contract-codegen#266, head 0a238ef, merge base e0f1005. The diff touches tests
only. In `tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t` it moves T from 1 s
to 3 s and the stand-in sleep from 2 s to 8 s, so the outer bound is 12 s. In
`a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return` it raises the
elapsed ceiling from 2 s to 20 s. Both doc comments are updated to match.

## Method

- Read the whole diff, `run_launcher`, `wait_until`, `kill_process_tree`, `capture`,
  `outer_bound` and `run_group`, then traced the old scenario (T = 1 s, sleep 2 s, bound 4 s)
  through them.
- Built the lib test binary at the head and at the merge base, each in its own worktree and
  target dir. Mutants and an instrumented copy were built in a third worktree.
- Reproduction on an 8-core host with swap full and ambient load 10 to 22:
  - Old test: 30/30 passed at ambient load. Under 150 CPU burners it failed 1 time in 140
    runs (30 + 110) with `BatchTimedOut { members: 4, timeout: 1s }`.
  - An instrumented copy of the old test ran 270 times under 150 burners, burner-start storms,
    and 60 burners plus 4 buffered 1 GiB writers. Its phase timings were: spawn to script
    start ≤ 39 ms, prologue ≤ 75 ms, `sleep 2` ≤ 2130 ms, tail ≤ 96 ms, total ≤ 2200 ms. It
    never reached 4 s.
  - New test: 10/10 passed at ambient load (8.02 s each) and 33/33 under 150 burners.
- Mutants, applied at the `run_group` call site:
  - bound = T: the new test fails (BatchTimedOut at 3 s).
  - bound = N·T/2: the new test fails (at 6 s).
  - bound = `Duration::MAX`: the outer-bound timeout test fails after 45 s.
  - bound = (N−1)·T: survives every test. Recorded in SR-1466.
  - Capture ignores the stop flag: the orphan test still passes, in 0.20 s.
  - Capture ignores the stop flag and no group kill: the orphan test fails after 45.0 s.
- Gates at the head: `make fmt-check` passed. `make lint` (clippy `-D warnings`) passed.
  `make audit-unsafe` passed. `make spec` exited 0 with two warnings that were already there
  (FR-017 line 152, EARS). The full lib binary passed 161/161. I did not run the whole
  `make ci`/msrv lane because the host was loaded.
- Production code: neither file changes outside its `#[cfg(test)]` module (execute.rs hunk at
  1500+ is inside `mod batch_tests` from 1043; launch.rs hunks at 532/546 inside `mod tests` from
  400). NFR-005 surface unchanged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The orphan test cannot detect the property it names. `( sleep 45 & )` leaves the orphan in the launcher's process group, so the group kill ends it at about 200 ms and the pipe reaches EOF. A capture that ignores the stop flag still passes in 0.20 s. The test fails only when the stop path and the group kill are both broken. The PR's new doc ("still under the 45 s") restates this false premise. The flaw predates the PR. | src/kani/run/launch.rs:531-554 |
| FND-002 | low | Pre-existing sibling with the same 2 s margin the PR removed elsewhere. After the stream goes over the limit, the over-limit test gives the kill 2 s (`sleep 2; touch marker`), and it fails if the marker exists. On a loaded host it has the same exposure the old 1 s / 2 s / 4 s test had. It passed 60/60 under 150 burners, so the risk is low. | src/kani/run/launch.rs:808-839 |
| FND-003 | low | The grandchild-kill ladder passes without proving anything when the first 200 ms rung ends before the grandchild writes its pidfile, because a missing pidfile counts as killed. That is the case most likely on a loaded host. The ladder makes the test robust but weak under load. Pre-existing. | src/kani/run/launch.rs:440-466 |

### FND-001 detail

Scenario: replace `if drain_until.is_none() && stop.load(Ordering::Acquire)` in `capture` with
`if false && ...`. The capture thread then reads until EOF. `a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return`
still passes in 0.20 s, because `kill_process_tree` SIGKILLs the orphaned `sleep 45` with the
rest of the group, which closes the last write end. To exercise FR-017-AC-16 at run level, the
orphan has to leave the group, for example `setsid sleep 45 &` or a `setsid`/`setpgid`
wrapper. The 20 s ceiling then really separates "returned after the drain limit" from "waited
for the orphan's 45 s". The PR's change to the ceiling is correct as far as it goes. This
finding is about the test's premise and does not block this PR.

### FND-002 detail

Order of events: `head -c 8MiB+1` finishes, a capture thread reads past the limit and sets
`failed`, `wait_until` sees it within 20 ms, and the group is killed. If the reading thread
and the waiter are starved for more than 2 s together, the shell runs `touch marker` and the
assertion `!marker.exists()` fails. Fix: use the same widening as this PR (for example
`sleep 10`), with the 120 s budget unchanged.

## Verdict

Approve. The test-only change is correct and is a real robustness improvement. The stand-in's
`sleep 8` cannot end early, so it outlasts T (3 s) by 5 s and N·T/2 (6 s) by 2 s. Only the
non-sleep overhead counts against the 12 s bound. That overhead measured at most 0.2 s
(spawn, prologue, sleep wake-up, JSON write, 20 ms poll) under 19× CPU oversubscription,
against the 4 s of headroom left (previously 2 s). The bound = T and bound = N·T/2 mutants
stay killed. The other half of FR-028-AC-12, a group still running at N·T, is killed by the
untouched 45 s / 200 ms test, which kills the `Duration::MAX` mutant. On the orphan test, the
20 s ceiling is still under the 45 s orphan, but see FND-001. The three findings all predate
the PR and none blocks the merge.

Rust-review lane: idioms, tracing tags (`Trace:` lines unchanged and resolvable), placement
(in-module `#[cfg(test)]`), panic surface (test-only `unwrap`), no `#[allow]`, no CI-workflow
change. The §1 "no clock-based assertions" rule is knowingly waived here: the unit under test
is a wall-clock bound with no clock seam, so a real-time test is the only end-to-end check.
Its residual flake risk is now bounded by a 4 s margin instead of 2 s.

## Dispositions

Round 1, reviewed at 871efbdbb91a882c3d55aca24b4ca6dfc7b90739 (fix commit 871efbd on top of
0a238ef; only src/kani/run/launch.rs changed, all inside `mod tests`).

Re-measured at that head:
- Results:
  - The four touched tests passed 10/10 with no `sleep 45` left behind.
  - The orphan, grandchild and over-limit tests each passed 25/25 under 150 CPU burners, with
    no `sleep 30` or `sleep 45` left behind.
  - The full lib binary passed 161/161.
  - `make fmt-check` and `make lint` passed.
- Mutant: the stop-flag-ignored mutant (separate worktree and target) fails the orphan test at
  45.0 s. While it ran, the escaped `sleep 45` had ppid 1 and pgid = sid = its own pid. It was
  killed by pid afterwards and nothing was left behind.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 871efbd |
| FND-002 | fixed | 871efbd |
| FND-003 | fixed | 871efbd |

FND-001 after: `( setsid sh -c 'echo $$ > {pid}; exec sleep 45' & ); until [ -s {pid} ]; do
sleep 0.02; done; exec sleep 45`, tried up the budget ladder. A rung with no recorded pid is
inconclusive, and the orphan is killed by pid before the assertions run.

FND-002 after: `sleep 45 & echo $! > {}; head -c {} /dev/zero; sleep 10; touch {}; wait`.

FND-003 after: `let killed = grandchild_pid.is_some() && !grandchild_survived;` then
`if killed { return; }`. A missing pidfile now moves to the next rung, and if every rung ends
without a recorded pid the test panics.

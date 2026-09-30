---
id: "SR-627"
title: "IR-93 slice 1 code review (incl. rust-review lane): precondition vacuity, kani lane witness join, hardened launcher"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@c5cdf7d396dc1ca1ba48c8b164f9149827d3030c; src/kani_execution.rs, Cargo.toml, Cargo.lock, Makefile"
relationships: []
---

# SR-627: IR-93 slice 1 code review

## Summary

Ticket: IR-93 (slice 1: IR-310, IR-354, IR-353). PR: agent-ix/quire-contract-codegen#202, head
c5cdf7d, diffed against origin/main with `git diff origin/main...HEAD`. The rust-review lane is
folded into this file.

The PR does three things. It exempts a precondition harness from the zero-checks vacuity rule and
lets its cover decide (IR-310). It adds `kani_witness_join` to the `make kani` filter (IR-354). It
rewrites `run_launcher_with_timeout` (IR-353): each stream keeps at most its last 8 MiB, the
capture threads poll and are always joined, the deadline uses `Instant::checked_add`, and
descendants are killed through pidfds using a new direct dependency, `rustix =1.1.5`.

## Method

I read the whole diff, `classify_transcript`, `render_precondition` in src/kani_obligations.rs and
the rustix 1.1.5 sources for `pidfd`, `poll` and `Timespec`. I checked the precondition premise
against a real prover: a scratch crate with a cover-only harness under `cargo kani` printed
`** 0 of 0 failed`, `** 1 of 1 cover properties satisfied` and `VERIFICATION:- SUCCESSFUL`. With an
unsatisfiable cover it printed the same except `** 0 of 1 cover properties satisfied`, which
`classify_transcript` maps to `CoverUnsatisfied`.

I ran `make ci` with a scratch `TRUSTED_HOME` holding symlinks to ~/.cargo and the nvm `quire`.
fmt-check, spec, clippy `-D warnings`, the msrv `cargo +1.98.1 test --locked` (85 lib and 213 `it`
tests passed, 5 ignored), `cargo deny check`, the one-copy awk, audit-unsafe and rustdoc all passed.
The final `make test` step did not start because another session deleted the shared scratch
`TRUSTED_HOME` directory. It is the same suite on the same pinned 1.98.1 toolchain, which had
already passed in the msrv step. I did not run `make kani`: the host load average was about 21 on
8 cores.

I ran one temporary reproduction test in my own review worktree, then reverted it. Its result is
under FND-003. The permission layer then refused further edits to the worktree, so the FND-004
mutations are argued from the code, not executed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The library now compiles on Linux only. rustix gates `pidfd_open` and `pidfd_send_signal` behind `cfg(target_os = "linux")`, and nothing in this crate is cfg-gated or documents a Linux-only platform. So the whole crate, not only the Kani path, fails to build on macOS and the BSDs. `tv_nsec: i64::try_from(..)` also mismatches rustix's `Nsecs = c_long` on 32-bit libc targets. | Cargo.toml:23, src/kani_execution.rs:42-47, src/kani_execution.rs:522-525, src/kani_execution.rs:565-580 |
| FND-002 | low | When `pidfd_open` fails (ENOSYS before Linux 5.3, or a seccomp profile that denies it), `kill_process_tree` drops every descendant through `.ok()?` and kills only the launcher. That is the pre-#58 failure, CBMC surviving a timeout, and it happens silently. The code it replaced worked wherever `kill` was on PATH. | src/kani_execution.rs:566-580 |
| FND-003 | low | The join after `stop` has no time bound. A capture thread stops only after a 20 ms poll that finds nothing, so a straggler the snapshot did not see, writing with no 20 ms gap, blocks the join and `run_launcher_with_timeout`. That contradicts the module doc's and the function doc's "returns within timeout plus a small constant either way". On main the timeout path never joined. | src/kani_execution.rs:11-13, src/kani_execution.rs:431-433, src/kani_execution.rs:475-477, src/kani_execution.rs:527-534 |
| FND-004 | low | `a_capture_thread_stops_on_request_while_the_pipe_is_still_open` cannot catch the mutations that matter. `stop` is true from the start. Deleting the `Ok(0) if stopping => break` arm makes the test hang, not fail, because libtest has no timeout. Moving the `Acquire` load after `poll`, or breaking on any empty poll whatever the flag, both still pass. | src/kani_execution.rs:1328-1333, src/kani_execution.rs:527-534 |
| FND-005 | low | Two doc comments are stale. The module doc still says `kill_process_tree` "signals each by its own positive pid"; it now signals pidfds. The function doc says the launcher "is stopped before the descendants are signalled, so it forks no more". But the snapshot and the pidfd opens come before `child.kill()`, so forks in that window are missed anyway. Also, the parent re-check drops a real descendant whose parent exited inside the window. | src/kani_execution.rs:14-17, src/kani_execution.rs:556-564, src/kani_execution.rs:574 |
| FND-006 | low | The two public classifiers now disagree on the same transcript. `classify_kani_run` hard-codes `kind = None`, so a precondition harness's `0 of 0` transcript is `VacuousProof` through it and `Verified` through `launch_evidence`. The real-capture tests in src/kani_transcript.rs go through `classify_kani_run`. | src/kani_execution.rs:676-678, src/kani_execution.rs:624-641 |

### FND-001 detail

In rustix-1.1.5/src/process/mod.rs, `mod pidfd` is `#[cfg(target_os = "linux")]`. `use
rustix::process::{pidfd_open, pidfd_send_signal, PidfdFlags, ..}` is unconditional at
src/kani_execution.rs:42-47, so `cargo build` on aarch64-apple-darwin fails with unresolved imports.
Before this PR the Linux-only parts (`/proc`, the `kill` binary) failed at run time and still
compiled. CI runs only on ubuntu-24.04, so nothing catches this.

Fix: either state that the crate is Linux-only (a `compile_error!` under
`cfg(not(target_os = "linux"))` plus a README line), or gate the pidfd path and use
`rustix::process::kill_process` elsewhere. A `const` `Timespec` built from literal fields also
removes the `unwrap_or(0)`, whose fallback would turn the poll into a busy spin.

### FND-002 detail

Fix: when `pidfd_open` returns ENOSYS or EPERM, fall back to `rustix::process::kill_process(pid,
Signal::KILL)` after the same parent re-check. Or count the descendants that could not be opened
and surface the count, so a partial kill is observable.

### FND-003 detail

Reproduction: `( sh -c 'while :; do echo x; done' & ) ; exec sleep 45` with a 300 ms budget. On
this host (load about 21) it returned `TimedOut` in about 0.5 s, because the orphan writer was
descheduled for more than 20 ms. The reader then stopped and dropped the read end, and the orphan
died of SIGPIPE. So I could not reproduce a hang here, and in practice the bound is the first
quiet gap of any surviving writer. On an idle machine a CPU-bound orphan that writes continuously
has no such gap.

Fix: bound the drain after `stop`, for example stop after N intervals or a fixed grace period
whether or not data is still arriving. Or weaken both doc comments to the bound the code actually
gives.

### FND-004 detail

Fix: run `capture_tail` on a thread with `stop` false. Have the writer write, sleep, and write
again, then set `stop` and assert that both writes were captured. Join through a channel with
`recv_timeout`, so a mutation that never stops fails the test instead of hanging it.

### FND-006 detail

Fix: give `classify_kani_run` a `kind` parameter, or make it `pub(crate)` and route the tests
through `launch_evidence`, so one rule has one entry point.

## Verdict

Not mergeable as-is: fix or explicitly accept FND-001, the medium. Nothing is high, and the logic
of all three tickets is correct.

What is right:

- IR-310 is sound. `checks_gate` is `None` only for `ObligationKind::Precondition`.
  Postcondition, Invariant, Frame and `None` (scalar harnesses) keep the zero-checks rule, and the
  new unit test covers Postcondition and `None`. A genuinely vacuous precondition, one whose cover
  cannot be satisfied, still cannot become `Verified`. Real Kani prints `0 of 1 cover properties
  satisfied` for it, which is `CoverUnsatisfied`. A missing summary or a `0 of 0` cover summary is
  `MissingCoverSummary`. `Verified` still needs banner SUCCESSFUL, exit 0 and `satisfied == total
  > 0`. The precondition harness asserts nothing beyond its cover (`render_precondition`,
  src/kani_obligations.rs:2157-2174), so skipping the checks count loses no property.
- IR-354: `kani_witness_join` is in the `make kani` filter, it matches
  `kani_witness_join::tc_026_…` (`#[ignore]`d, tests/it/kani_witness_join.rs:354), and the Makefile
  comment is corrected.
- IR-353 mechanics:
  - The tail cap keeps memory at 2 × limit plus 64 KiB at most.
  - `checked_add` removes the `Duration::MAX` panic.
  - Both capture threads are joined on every outcome.
  - The launcher is reaped with `wait` on the kill path and `try_wait` on the completed path.
  - pidfd plus the parent re-check is safe against pid reuse: a pidfd opened on a reused pid names
    the new process, whose parent will not match, and a process that exits after `pidfd_open`
    makes the signal a no-op.
  - The integer conversions (`i32::try_from(pid).ok()?`, `i64::try_from(nanos)`) are checked.
  - There is no unsafe code, and `#![forbid(unsafe_code)]` holds.
  - The completed path is better than main: a silent straggler holding the pipe no longer blocks
    `read_to_end` forever.
- The ticket text of IR-353 (untrusted Linear data) says "run() (:1003) probes versions with
  .output() and no timeout". That does not match the code: `git grep '\.output()'` finds no hit
  under src/ at origin/main or at the head. The coder's claim is confirmed.
- rustix 1.1.5 was already in Cargo.lock as a transitive dependency, so the PR adds no new crate
  and no second copy. `cargo deny check` and scripts/check_one_copy.awk pass. The dependency is
  justified: std has no `poll`, and `std::os::linux::process::PidFd` is unstable, so under
  `forbid(unsafe_code)` `libc` is not an option.
- `.gitignore:15` `target-*/` matches `target-codex-backends/` (checked with `git check-ignore -v`).
  The brief's claim that the lane's target directory is not ignored is incorrect.

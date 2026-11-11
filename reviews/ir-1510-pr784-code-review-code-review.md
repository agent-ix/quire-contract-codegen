---
id: SR-17940486709564847047667
title: "IR-1510 PR #784 minimal Kani runner code review"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #784 @ 5889a7b6; IR-1510; src/kani/run/{launch,owners,memory,execute,process,tool,mod}.rs, src/kani/run/execute/two_phase_tests.rs, src/kani/{classify,terminal}.rs, src/routed/smt/mod.rs, src/lib.rs, tests/it/{kani_run_scan,terminal_map,kani_terminal_private}.rs, Cargo.toml, Cargo.lock, CLAUDE.md, spec/kani/** and spec/spec.md, spec/tests.md edits"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-045
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/ADR-007
    type: reviews
---

## Summary

Ticket: IR-1510. PR #784 (head 5889a7b6, branch code/ir-1510-minimal-kani-runner), scoped to `git diff origin/main...HEAD` (33 files, +1051/-3989). Methods: code-review with the rust-review lane folded in (Rust diff). Reviewed read-only in /home/peter/dev/worktrees/IR-1510-review. Context: ADR-007 minimal run ownership (own process group, wall-clock timeout plus group kill, peak only from an environment source, environment owns the memory cap, macOS-capable), spec merged in CG #782.

Launch path checked. `run_monitored_with` sets `process_group(0)`. `wait_until` polls with `waitid(EXITED|NOHANG|NOWAIT)`, so the launcher is never reaped while it is polled. Every conclusion (completed, timeout, cancel, capture failure, wait error) then runs `signal_group`, which does `kill_process_group(pgid, KILL)` plus `child.kill()`, before `reap`. The unreaped launcher (a zombie at worst) keeps the pgid reserved, so the group signal cannot reach a recycled group. Grandchildren that stay in the group are killed on every path, including after the launcher exits on its own. Descendants that call setsid escape the kill; ADR-007 accepts this. rustix 1.1.6 compiles `waitid`, `WaitIdOptions::NOWAIT` and `kill_process_group` for apple targets. Nothing on the launch path is Linux-only: the `/proc` and `cfg(target_os = "linux")` uses left in `src/kani/run/launch.rs` are inside the `#[cfg(test)]` module. command-fds, nix and cfg_aliases are gone from Cargo.toml and Cargo.lock, and nothing else in the tree uses them. Makefile, scripts, .github and CLAUDE.md hold no bwrap, bubblewrap, unshare, pidfd or cgroup residue. `KaniInstallation::with_peak_memory_source` is public, and `MemoryObservation::observe` converts KiB to bytes with an overflow check, giving None on overflow.

Reviewer test job, flock'd, `--locked`, own target dir, no Kani: lib `kani::run:: kani::terminal kani_terminal_private` 131 passed; it `kani_run_scan terminal_map` 35 passed. Mutation control: in `SystemTeardown::signal_group`, the `kill_process_group` call was replaced by a no-op and the `tc_043` launch tests were rerun. Three failed: `tc_043_a_timeout_kills_the_whole_group_and_returns_within_the_allowance` (AC-66), `tc_043_a_launcher_that_exits_on_its_own_has_its_grandchild_killed` (AC-24) and `tc_043_an_over_limit_run_is_stopped_and_kills_the_launcher_group` (AC-14). The mutation was reverted.

## Verdict

**CHANGES REQUIRED.** The core mechanism is correct and minimal. Group kill before reap holds on every path, there are no Linux-only constructs, the peak seam is right, and the AC-65, AC-66 and AC-68 controls can fail. Two high findings block merge:

- FR-017-AC-63(c) says a witness-phase `out_of_memory` report is `FailedWithoutCounterexample`, but `conclude_witness` turns it into `WitnessDisagreed`, and no test covers that case.
- FR-045-AC-3 still promises a memory `Stopped` limit that the code can no longer produce.

Several spec and matrix rows also still describe removed mechanisms or overstate coverage.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-017-AC-63(c) requires a witness-phase report stating exit status `out_of_memory` to be `FailedWithoutCounterexample` with cause `out_of_memory` (FR-028-AC-25), with no third launch and equal recorded ceilings. `conclude_witness` instead compares failed properties. The proof failed FAIL_10, and an out-of-memory report has no checks (the `failing_with` fixture shape is `&[]`), so `same_failures` is false. The witness is relabelled `WitnessDisagreed` with `failure_cause` cleared, which FR-029 maps to `Failed`, not `Incomplete(ResourceExhausted)`. On main, the runner's `MemoryExhausted` arm covered this case; IR-1510 removed that arm and added no out_of_memory witness control. The new `a_witness_phase_killed_by_the_environment_is_no_verdict` covers only the SIGKILL no-report shape and does not assert the ceilings. | src/kani/run/execute.rs:675-715, src/kani/run/execute/two_phase_tests.rs (a_witness_phase_killed_by_the_environment_is_no_verdict) |
| FND-002 | high | FR-045-AC-3 still says the selected solver has finite per-query memory limits and that "a reached limit gives `Stopped` with that limit". IR-1510 removed `ProcessOutcome::MemoryExhausted` and `MemoryUnobserved` and deleted the FR-045-AC-3 test `owned_child_tree_stops_when_memory_limit_is_reached`. `ReachedLimit::MemoryBytes` is now constructed nowhere, and `ProcessLimits.memory_bytes` is documented as unenforced. FR-045 was not amended, so the spec states a behaviour the code cannot produce. | src/routed/smt/mod.rs:28, src/routed/smt/mod.rs:205-216, src/kani/run/process.rs:24-26, spec/routed/functional/FR-045-hyper-smt-induction-map.md:208 |
| FND-003 | medium | Spec text still describes removed mechanisms or uses future tense for IR-1510. (1) FR-017 Behavior `class()` lists `MemoryMechanismUnavailable`, `MemoryObservationFailed` and `BatchMemoryExhausted` as live `EnvironmentFault` refusals. (2) FR-017-AC-63(c): "the tests tagged for it are IR-1510's to retag or delete". (3) FR-029 Behavior: "Until IR-1510 removes the runner's own memory stop". FR-029-AC-34: "as does the memory-exhausted reason until IR-1510 removes it". FR-029 Status: "the map also carries a `MemoryExhausted` arm". (4) interface-001 line 134 says the playback "`MemoryExhausted { memory }` outcome is current code, removed by IR-1510", and line 363 still describes a timeout kill that reaches "every process it forked that a `/proc` walk taken at that moment can still see". (5) TC-039 Status: "the runner-side mechanism that IR-1510 removes". | spec/kani/functional/FR-017-kani-execution-evidence.md:816-820, spec/kani/functional/FR-017-kani-execution-evidence.md:914, spec/kani/functional/FR-029-run-outcome-terminal-record.md:186, spec/kani/functional/FR-029-run-outcome-terminal-record.md:414, spec/kani/functional/FR-029-run-outcome-terminal-record.md:454-456, spec/core/functional/interface-001-codegen-api.md:134, spec/core/functional/interface-001-codegen-api.md:363, spec/kani/matrix/TC-039-bounded-proof-ceilings.md:219 |
| FND-004 | medium | The PR deleted `observed_memory_exhaustion_maps_to_incomplete_resource_exhausted`, the only test tagged FR-029-AC-3, and `quoin matrix` now computes FR-029-AC-3 as untagged with 0 binders. The kani matrix row still reads "✅ Covered; timed-out, exhausted-unwind-bound and observed memory-exhausted reasons are asserted". FR-029-AC-3 still states the memory-exhausted mapping. The surviving `tc_040_a_timeout_and_an_exhausted_unwind_bound_are_incomplete` carries only `TC-040`. | tests/it/terminal_map.rs:224-236, spec/kani/matrix/tests.md:88, spec/kani/functional/FR-029-run-outcome-terminal-record.md:384 |
| FND-005 | medium | The kani matrix FR-017-AC-19 row was flipped to "✅ Tagged (IR-1510)", but no test carries FR-017-AC-19: `quoin matrix` shows it untagged with 0 binders. The row text still says "no test carries the FR-017-AC-19 tag ... IR-1510 tags them". | spec/kani/matrix/tests.md:62 |
| FND-006 | medium | Public API removals break a downstream crate that tracks CG `branch = "main"`. quire-driver origin/main imports `MemoryMechanism` and builds `MemoryObservation { mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss, .. }` (src/provider.rs:452,561 in tests; tests/drive.rs:37,1094). Its next `cargo update` fails to compile those tests. `KaniInconclusiveReason::MemoryExhausted`, `ProcessOutcome::{MemoryExhausted, MemoryUnobserved}`, `ProcessError::Unavailable` and `ProcessFailurePhase::{Ownership, Startup}` are also removed, and the evidence wire shape loses `memory.mechanism`. The PR does not call out the breaking change. | src/lib.rs:322-323, src/kani/classify.rs:48-52, src/kani/run/process.rs:59-125 |
| FND-007 | low | The FR-017-AC-58 wording is inaccurate. FR-017 "Source changes" says the playback `MemoryExhausted { memory }` outcome is "gone", and AC-58 says "IR-1510 removed that variant". Neither main nor the PR has `execute_kani_concrete_playback` or `KaniConcretePlaybackOutcome`: the playback is PLANNED under IR-917 and the variant never existed. | spec/kani/functional/FR-017-kani-execution-evidence.md:838-842, spec/kani/functional/FR-017-kani-execution-evidence.md:909 |
| FND-008 | low | The AC-67 scanner's exclusions are wider than the spec. (1) `is_cfg_test` treats any `cfg(..)` whose text contains "test" as test-only, so production items under `cfg(not(test))` or `cfg(feature = "test-support")` would be skipped. (2) `visit_attribute` drops every attribute, while the spec excludes only `#[doc]`. (3) The "comment" control counts token occurrences in the FIXTURE string and never runs a comment-including scanner, so it cannot fail for a scanner reason. | tests/it/kani_run_scan.rs:76-80, tests/it/kani_run_scan.rs:126-127, tests/it/kani_run_scan.rs:247-256 |
| FND-009 | low | The AC-68 test's "mutant" block drives a `RecordingTeardown` by hand (reap, then signal) and asserts that the recorder logs what the test itself just called. That is a tautology. The real control is the production run asserting `["signal_group", "reap"]`, which a reap-first mutant would fail. The block adds no evidence and reads as if it did. | src/kani/run/launch.rs:1731-1742 |
| FND-010 | low | `execute_kani_obligation_readonly(request, source_directory)` now uses `source_directory` only as `Some(_)`, to add `--lib`. The parameter name `read_only_source` in `run_single` and `run_phase_single` no longer means read-only. FR-017 "Source changes" records that the runner no longer re-binds the tree read-only, but ADR-007 (Consequences) says nothing about it. | src/kani/run/execute.rs:406-421, src/kani/run/execute.rs:729-760, spec/decisions/ADR-007-minimal-kani-run-ownership.md |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-017-AC-65 | examined | The launcher is started as the leader of a new process group, so the group id a launcher stand-in reports for itself equals its own process id and differs from the group id of the calling test process |
| FR-017-AC-66 | examined | When the wall-clock timeout elapses, a launcher stand-in that has started a descendant in its group (and a grandchild) ... is killed together with every one of them, and the run returns TimedOut within the timeout plus RETURN_ALLOWANCE of 1 second |
| FR-017-AC-67 | examined | A structural scan of the non-test, non-comment source files of src/kani/run/ ... finds none of the tokens bwrap, bubblewrap, unshare, CLONE_NEW, pidfd, procfs, /proc, cgroup and namespace |
| FR-017-AC-68 | examined | When the run kills the launcher's process group, it signals the group before it reaps the launcher |
| FR-017-AC-63 | examined | (c) ... one whose report states the exit status out_of_memory ... is FailedWithoutCounterexample with cause out_of_memory (FR-028-AC-25), with no third launch and with recorded ceilings equal to the proof launch's |
| FR-017-AC-58 | examined | a playback process the environment ends for memory returns Completed with succeeded false ... with no MemoryExhausted playback outcome |
| FR-017-AC-19 | examined | matrix row status flip only |
| FR-028-AC-21 | examined | ENVIRONMENTAL (ADR-007). The caller's environment owns the memory cap ... Where the source provides a peak, the evidence records it with the source's name; where it provides none, the evidence records no peak |
| FR-028-AC-25 | examined | A report whose entry for the harness states the exit status out_of_memory ... is FailedWithoutCounterexample with failureCause out_of_memory ... A launcher stand-in that is killed by SIGKILL and exports no report ... is NoVerdict |
| FR-028-AC-2 | examined | A backend Kani run that exceeds its wall-clock ceiling is stopped and classified KaniRunOutcome::Inconclusive with the timed-out reason |
| FR-029-AC-3 | examined | The timed-out reason maps to Incomplete(TimedOut), and the memory-exhausted (removed by IR-1510; ADR-007) and exhausted-unwind-bound reasons each map to Incomplete(ResourceExhausted). |
| FR-029-AC-34 | examined | as does the memory-exhausted reason until IR-1510 removes it |
| FR-045-AC-3 | examined | finite per-query input-byte, wall-clock, memory and captured-output limits ... a reached limit gives Stopped with that limit |
| FR-040-AC-5 | examined | A multi-member group sharing one handle stops wholly on its observed cause, returns BatchCancelled { source, members } ... another group using a distinct handle retains its own result |
| ADR-007 | context_only | minimal Kani run ownership: own process group, wall-clock timeout and group kill, peak only from the environment, environment owns the memory cap |
| interface-001 | examined | lines 134 and 363 (playback outcome, timeout kill wording) |

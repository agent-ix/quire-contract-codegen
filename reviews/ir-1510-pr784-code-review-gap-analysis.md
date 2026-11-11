---
id: SR-17940486709564847047668
title: "IR-1510 PR #784 minimal Kani runner gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #784 @ 5889a7b6; IR-1510; FR-017-AC-19, FR-017-AC-58, FR-017-AC-63, FR-017-AC-65 to FR-017-AC-68, FR-028-AC-3, FR-028-AC-21, FR-028-AC-25, FR-029-AC-3, FR-040-AC-5, FR-045-AC-3; TC-027, TC-039, TC-040, TC-043, TC-055; src/kani/run/**, tests/it/kani_run_scan.rs, tests/it/terminal_map.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-043
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-039
    type: reviews
---

## Summary

Ticket: IR-1510. Plan completion: not assessed (planless). Inputs: `quoin matrix --json` (quoin 0.29.1) at 5889a7b6, the tagged tests, and the source.

The matrix at the PR head:
- FR-017-AC-65 is tagged with 1 binder, AC-66 with 4 and AC-68 with 1.
- FR-017-AC-67 has 2 binders and status method-without-symbol, because its method is Analysis.
- FR-028-AC-21 is tagged with 3 binders and FR-028-AC-25 with 2.
- FR-028-AC-3 and FR-017-AC-55 are untagged (retired, as intended).
- FR-017-AC-19 is untagged (0 binders), FR-029-AC-3 is untagged (0 binders), and FR-017-AC-58 is untagged (PLANNED, IR-917).
- FR-045-AC-3 still has 14 binders.

Reviewer test job, flock'd, `--locked`, own CARGO_TARGET_DIR, no Kani:
- `cargo test --lib -- kani::run:: kani::terminal kani_terminal_private`: 131 passed, 0 failed.
- `cargo test --test it -- kani_run_scan terminal_map`: 35 passed, 0 failed.

Mutation control: in `SystemTeardown::signal_group`, the `kill_process_group(group, Signal::KILL)` call was replaced by a no-op. Three tests then failed: `tc_043_a_timeout_kills_the_whole_group_and_returns_within_the_allowance` (AC-66), `tc_043_a_launcher_that_exits_on_its_own_has_its_grandchild_killed` (AC-24) and `tc_043_an_over_limit_run_is_stopped_and_kills_the_launcher_group` (AC-14). The other 10 tc_043 launch tests passed. The mutation was reverted.

Each flipped AC was checked for a failing control:
- **AC-65:** a real `process_group(0)` spawn whose stand-in prints its pid and pgid (via `ps`, which also exists on macOS), with a caller-group control.
- **AC-66:** heartbeat files for the descendant and the grandchild, a 1 s return allowance, a 5 s pipe-holder, and the launcher-only-kill and self-ending controls. The reviewer's mutation killed it.
- **AC-67:** a syn scan over the files reachable from `src/kani/run/mod.rs` through non-test `mod` items. It checks identifiers and string literals, including macro tokens, against the exact nine-token list. The fixture self-test asserts 1, 0 and 2 findings for the full scan, the no-strings scan and the test-items scan.
- **AC-68:** a recording teardown seam around the production `run_monitored` on the timeout path and the self-exit path.
- **FR-028-AC-21:** a 64 MiB allocation under a 1 MiB ceiling stays `Verified`; a named fixed source gives bytes, a KiB source is converted, an overflowing reading gives None, and no source gives None with no name.
- **FR-028-AC-25:** an `out_of_memory` report gives `FailedWithoutCounterexample` with cause out_of_memory; a SIGKILL with no report gives `NoVerdict` with exit_code None.

Deleted tests:
- The FR-028-AC-3 `MemoryExhausted` tests, the FR-017-AC-55 namespace tests and the procfs-sampler tests that carried FR-028-AC-21 are deleted as ADR-007 directs.
- `prepared_source_bind_is_read_only_to_the_backend`, tagged FR-017-AC-28, is deleted. AC-28 concerns request identity, and its other binders remain in scalar.rs and kani_obligations.rs.
- `controlled_caller_order_distinguishes_cancelled_and_completed_batch` (TC-055, FR-040-AC-3, FR-040-AC-5) is deleted. FR-040-AC-5's cause-order reversal and independent-handle group stay covered by `cancelled_batch_retains_first_source_and_independent_group` and `distinct_cancellation_handles_have_distinct_launchers`.

## Verdict

**CHANGES REQUIRED (coverage and status).** AC-65 to AC-68 and FR-028-AC-21 have tests that can fail. FR-028-AC-25 has its single-phase controls. The gaps:

- The FR-017-AC-63(c) witness `out_of_memory` case has no test and does not hold in the code (code-review FND-001).
- Two matrix rows claim coverage that the computed matrix contradicts: FR-017-AC-19 and FR-029-AC-3.
- FR-045-AC-3 lost its memory-limit binder, and the criterion was not amended.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-017-AC-63(c), the witness-phase `out_of_memory` report, has no test. ADR-007 made the report-based classification the only memory classification, and IR-1510 owned retagging the (c) tests. It replaced the runner-memory witness tests with a SIGKILL no-report test, which asserts neither the `out_of_memory` cause nor the equal recorded ceilings. Traced through `conclude_witness`, the case gives `WitnessDisagreed` (see code-review FND-001). | src/kani/run/execute/two_phase_tests.rs (a_witness_phase_killed_by_the_environment_is_no_verdict), src/kani/run/execute.rs:675-715 |
| FND-002 | medium | FR-029-AC-3 lost its only binder: the deleted `observed_memory_exhaustion_maps_to_incomplete_resource_exhausted`. `quoin matrix` computes it untagged, but the kani matrix row says "✅ Covered ... observed memory-exhausted reasons are asserted". The surviving timed-out/unwind test is tagged `TC-040` only. | tests/it/terminal_map.rs:224-236, spec/kani/matrix/tests.md:88 |
| FND-003 | medium | The FR-017-AC-19 matrix row was flipped to "✅ Tagged (IR-1510)", but the computed matrix shows 0 binders. The two named tests were never tagged. | spec/kani/matrix/tests.md:62 |
| FND-004 | medium | The FR-045-AC-3 memory-limit binder `owned_child_tree_stops_when_memory_limit_is_reached` was deleted, and `ReachedLimit::MemoryBytes` is unreachable. The criterion's memory clause now has no code that could satisfy it (see code-review FND-002). | spec/routed/functional/FR-045-hyper-smt-induction-map.md:208, src/routed/smt/mod.rs:28 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-017-AC-65 | examined | The launcher is started as the leader of a new process group ... A stand-in left in the caller's group reports the caller's group id and fails. |
| FR-017-AC-66 | examined | ... killed together with every one of them, and the run returns TimedOut within the timeout plus the return allowance RETURN_ALLOWANCE of 1 second |
| FR-017-AC-67 | examined | A structural scan of the non-test, non-comment source files of src/kani/run/ ... finds none of the tokens ... including inside a string such as Path::new("/proc") |
| FR-017-AC-68 | examined | it signals the group before it reaps the launcher ... a mutant that reaps the launcher first and signals the group after records the reap first and fails the control |
| FR-017-AC-63 | examined | (c) ... one whose report states the exit status out_of_memory ... is FailedWithoutCounterexample with cause out_of_memory (FR-028-AC-25), with no third launch and with recorded ceilings equal to the proof launch's |
| FR-017-AC-19 | examined | matrix row "✅ Tagged (IR-1510)" |
| FR-017-AC-58 | examined | PLANNED (IR-917) ... with no MemoryExhausted playback outcome |
| FR-028-AC-3 | examined | RETIRED WITH REPLACEMENT FR-028-AC-25 (ADR-007) |
| FR-028-AC-21 | examined | Where the source provides a peak, the evidence records it with the source's name; where it provides none, the evidence records no peak |
| FR-028-AC-25 | examined | A report whose entry for the harness states the exit status out_of_memory ... is classified ... FailedWithoutCounterexample with failureCause out_of_memory ... A launcher stand-in that is killed by SIGKILL and exports no report ... is NoVerdict |
| FR-029-AC-3 | examined | The timed-out reason maps to Incomplete(TimedOut), and the memory-exhausted ... and exhausted-unwind-bound reasons each map to Incomplete(ResourceExhausted). |
| FR-040-AC-5 | examined | Reversing the two cause calls changes the cancelled group's source and not the other group's evidence. Distinct handles never share one launcher. |
| FR-045-AC-3 | examined | a reached limit gives Stopped with that limit |
| TC-055 | context_only | live Kani cancellation provenance; matrix status Planned |

## Dispositions

Round 1, verified at 93eaf66f with `quoin matrix --json` and the reviewer test job (see SR-17940486709564847047667 Dispositions).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `a_witness_report_stating_out_of_memory_is_failed_without_counterexample` (FR-017-AC-63, FR-028-AC-25) asserts `FailedWithoutCounterexample` with cause `out_of_memory`, that no third launch is made, and that the proof and witness ceilings are equal, plus a `WitnessDisagreed` control. Reverting the fix makes it fail. |
| FND-002 | fixed | The matrix shows FR-029-AC-3 tagged by `tc_040_a_timeout_and_an_exhausted_unwind_bound_are_incomplete`, and tests.md:88 agrees. |
| FND-003 | fixed | The matrix shows FR-017-AC-19 tagged with 2 binders (report_file.rs and execute.rs tc_027 tests), and tests.md:62 agrees. |
| FND-004 | fixed | FR-045-AC-3 no longer has a memory clause. Its 14 remaining binders (process.rs, pair_tests.rs) cover the input, wall-clock and capture limits it now states. |

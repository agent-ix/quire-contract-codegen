---
id: SR-792001
title: "IR-1514 PR #792 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #792 at e27b95b7; IR-1514; src/routed/smt/refinement.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-045
    type: reviews
---

## Summary

Ticket: IR-1514. PR #792 (one commit e27b95b7 on origin/main a52a000e, branch code/ir-1514-flaky-refinement-test) changes only the test `routed::smt::refinement::tests::non_proof_solver_outcomes_refuse_refinement_handoff` (src/routed/smt/refinement.rs:644-708). Each of the six fake solver scripts now begins with `cat >/dev/null;`, and the empty `:` script became `cat >/dev/null`. A two-line comment states why. The expected statuses, output limits, both assertions, the query, the limits and the `#[trace("TC-059", "FR-045-AC-7")]` tag are byte-identical to main. No production code changed. Reviewed read-only in /home/peter/dev/worktrees/IR-1514-review, scoped to `git diff origin/main...HEAD`. The rust-review lane is folded in here.

Root cause, confirmed by reading the runner: `spawn_input` (src/kani/run/launch.rs:606-647) writes the query on a nonblocking pipe. If the child exits before the write completes, the writer reports an error or stops with `Ok(false)`, which becomes `input_failure` (launch.rs:440-447). That makes `ProcessOutcome::InputUnread` (src/kani/run/process.rs:222-233), which `run_solver_query` maps to `SolverStatus::ProcessFailed` (src/routed/smt/mod.rs:212-214) ahead of the scripted outcome. Scripts that never read stdin therefore race the write.

Reviewer reproduction. The focused test binary was run with all 32 cores saturated by 32 busy loops; all loops were killed afterwards.
- At origin/main (test file checked out from main): 0/30 passed. The first case failed with `printf 'sat\n'`, left `ProcessFailed`, right `Sat`.
- At head e27b95b7: 100/100 passed.
- Mutation (`cat >/dev/null;` removed from the `sat` script only): 0/30 passed, with left `ProcessFailed`, right `Sat`. Each script needs its own drain, and the mutated test fails.

## Verdict

**MERGEABLE.** The fix is minimal and correct, and all six outcomes are still reached deterministically.
- `Sat` and `Unknown` are reached through a successful exit with parsed stdout.
- `ProcessFailed` now comes only from `exit 3`. Before, it could also come from the InputUnread race, which masked the case, so this case is stronger than before.
- The empty response (`cat >/dev/null`, exit 0, empty stdout) gives `MalformedResponse`.
- `invalid` gives `MalformedResponse`.
- `Stopped(OutputBytes)` is unchanged, because output over the limit is classified before completion.

No assertion was weakened.

Portability: `cat >/dev/null` is POSIX, under `/bin/sh` on Linux (dash) and macOS (bash in sh mode). The runner closes stdin once the query is written, so `cat` reaches EOF and does not hang. The suite already relies on the same idiom: `cat >&2` in `run_with_proof` (refinement.rs:354) and in refinement_source_tests.rs:287, 622 and 729.

Other fake scripts:
- Every other `run_solver_query` and `run_bounded_process` caller that sends a non-empty query uses a child that reads stdin (`/usr/bin/cat`, or `cat >&2`).
- The `/usr/bin/true`, `/usr/bin/false` and python callers send empty stdin. `spawn_input` returns `Ok(true)` at once for those (offset == len), so they have no race.
- The Kani launch tests do not feed stdin.

No other test in the crate carries this race.

Rust surface: test-only change, `rustfmt --check` is clean, and there is no new `unwrap`, `expect` or `unsafe` outside the existing test code. The trace attribute is untouched, and no `/// Trace:` line was added. No pin, hash or digest was added.

Coder's aside: `production_refinement_live_solver_handoff_reaches_qsl_checker` fails without `QUIRE_CVC5_BIN`. This is by design: it calls `expect("live refinement qualification requires QUIRE_CVC5_BIN")` at refinement_source_tests.rs:455. It is not related to this PR.

The solver-level coverage of the InputUnread mapping is recorded in SR-792002.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| src/routed/smt/refinement.rs | examined | src/routed/smt/refinement.rs:644-708 | non_proof_solver_outcomes_refuse_refinement_handoff, six cat >/dev/null scripts |
| src/routed/smt/mod.rs | context_only | src/routed/smt/mod.rs:170-222 | run_solver_query outcome to SolverStatus mapping |
| src/kani/run/launch.rs | context_only | src/kani/run/launch.rs:390-447, 606-647 | spawn_input nonblocking writer and input_failure |
| src/kani/run/process.rs | context_only | src/kani/run/process.rs:222-256 | InputUnread precedence over Completed |
| src/routed/smt/pair_tests.rs | examined | src/routed/smt/pair_tests.rs:41-290 | other fake-solver callers: /usr/bin/cat, or empty stdin |
| src/routed/smt/refinement_source_tests.rs | examined | src/routed/smt/refinement_source_tests.rs:275-290, 610-625, 729 | cat >&2 scripts |

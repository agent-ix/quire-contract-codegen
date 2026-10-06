---
id: SR-1720
title: "IR-241 PR 302 portability fix code review (Rust lane)"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@1a556e1b8cab6e9c0b7da812f52237cbbefd068d; src/kani/run/execute.rs, src/kani/run/launch.rs, src/kani/run/memory.rs, src/kani/run/namespace.rs against base 7889182361228727b2c606c7b922520c92a76870"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
---

# SR-1720: IR-241 PR 302 portability fix code review (Rust lane)

## Summary

Ticket: IR-241. PR: quire-contract-codegen#302. Method: code-review, with the rust-review lane
in this one artifact. Reviewer model: claude-opus-5-5. The diff is four files under
`src/kani/run/`, with 468 insertions and 111 deletions against `7889182`. No Cargo, Makefile,
workflow, spec or doc file changed.

The production change moves Linux-only imports and items behind `#[cfg(target_os = "linux")]`:
rustix `pipe_with`/`PipeFlags`, `command_fds`, serde `Deserialize`, `StartupInfo`,
`NamespaceOwner.info`, `read_info`, `startup_information`, `confirm_startup_group_dead`,
`startup_expiry`, `MemoryObserver::bind_root` and the rustix `Pid` import. It replaces the two
runtime `cfg!` checks with compile-time split bodies. The rustix items that remain
unconditional are `poll`, `PollFd`, `kill_process_group`, `Pid`, `Signal` and `Timespec`.
These are portable POSIX APIs.

On a non-Linux target, `run_bounded_launcher_at` still calls `MemoryObserver::prepare` first.
That returns `Unsupported`, which maps to `KaniExecutionRefusal::MemoryMechanismUnavailable`
before `NamespaceOwner::prepare` or any spawn. This covers both single and batch execution
(`start` is shared).

`run_single` and `run_group` were factored into `finish_single` and `finish_group`, which
return a memory-free private `ReportedExecution`. Measured memory is attached with
`with_memory` only after `start` returns a `BoundedLaunch`. The production order is unchanged:
fresh report path, stale removal, launch command, start, `take_report`, `settle`, then
classification. Error precedence is also unchanged, because `report?` is still evaluated after
`settle`.

The Linux namespace sequence is unchanged in `prepare`, `dispatch` and `cleanup`. That covers
argv order, fd mappings, the startup cap, the claim before gate release, and the group kill
before gate close. Kani harness selection is untouched. The coverage consequences of moving 15
tests onto the report fixture are recorded in SR-1721, not here.

## Method

- Read the repo `CLAUDE.md`, `clippy.toml`, `rustfmt.toml` and `deny.toml` conventions, then
  read every hunk of `git diff 7889182 1a556e1` and the surrounding code in
  `launch.rs:140-350` and `namespace.rs:1-450`.
- Checked each `cfg` gate for unconditional uses of Linux-only APIs (rustix pipe and pidfd,
  procfs reads, `command_fds`, serde startup decode).
- Traced the non-Linux refusal path through `start`, `run_bounded_launcher_at` and
  `MemoryObserver::prepare`.
- Compared the before and after production order of `run_single` and `run_group`.
- Checked the 10 newly guarded tests, the 1 non-Linux refusal test, the new planner test, the
  `report_fixture` module and the public-wire assertion added to the Linux N-to-1 count test.
- No build, cargo or Kani run was performed, as the reviewer brief instructs. The gate
  receipts are the root's measurements (Linux `make ci`, Linux `make kani` 25/25, and Draco
  native macOS check, Clippy and 178 library tests). They were supplied as data and this
  review did not re-execute them.

## Verdict

**PASS WITH LOW FINDINGS.** The portability fix is a compile-time split with no fallback. Linux
ownership and ceilings are not altered, and unsupported platforms refuse with the typed
reason before dispatch. Three low findings are listed below. The medium coverage findings are
in SR-1721 (gap-analysis), and the PR's overall review outcome depends on those.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `NamespaceOwner::dispatch` names its parameters `_deadline` and `_observer`, then uses both throughout the Linux body. The leading underscore says "unused", which is false on the only platform that runs the body. | src/kani/run/namespace.rs:188-250 |
| FND-002 | low | The new test `unequal_memory_ceilings_plan_separate_groups_with_their_own_selections` asserts `!calls.exists()` ("planning never dispatches a backend"), but nothing in the test can spawn the stand-in. The assertion cannot fail. | src/kani/run/execute.rs:1578-1610 |
| FND-003 | low | `ReportedExecution` restates the 13 non-memory fields of `KaniExecutionEvidence` and their serde attributes (`rename_all`, `skip_serializing_if` on `batch`) by hand, and `with_memory` maps them by name. The wire contract now lives in two places, and the portable FR-017-AC-22 wire assertion checks the private copy. | src/kani/run/execute.rs:260-298, src/kani/run/execute.rs:217-258, src/kani/run/execute.rs:2354-2361 |

### Failure scenarios

- FND-001: a later edit adds a Linux-only parameter use or removes one. Clippy cannot report a
  genuinely unused `_deadline`, because the underscore suppresses `unused_variables` on every
  platform, Linux included. Fix: follow the PR's own `MemoryObserver::prepare` pattern
  (`memory.rs:56-66`), with two `cfg`-split `dispatch` functions. Alternatively, keep the real
  names and put `#[cfg_attr(not(target_os = "linux"), allow(unused_variables))]` on the method,
  with a reason comment.
- FND-002: delete `plan_groups` entirely and replace it with an implementation that spawns.
  The assertion still holds, because the test never calls anything that could reach the
  stand-in. The only signal it gives is that `StandIn::verifying` did not run itself. Fix:
  drop the tautological assertion, or assert the property that matters, for example that
  `shares_process` is false for the two requests.
- FND-003: change `KaniExecutionEvidence.batch` to drop `skip_serializing_if`, or rename a
  field's wire key on the public struct only. The portable test at `execute.rs:2354-2361`
  still passes, because it serializes `ReportedExecution`. On macOS no test serializes the
  public evidence at all. On Linux only `batch.members` and `batch.timeoutSeconds` are checked,
  in the N-to-1 count test. Fix: keep one wire definition. Either derive the public record from
  the memory-free one through a single source (for example, the portable test asserts on
  `with_memory` output built from an observation the test owns), or drop `Serialize` from
  `ReportedExecution` and move the wire assertions to the public type on the Linux path. See
  SR-1721 FND-001.

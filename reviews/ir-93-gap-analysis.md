---
id: "SR-628"
title: "IR-93 slice 1 gap analysis: FR-017 against the precondition, witness-join and launcher changes"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@c5cdf7d396dc1ca1ba48c8b164f9149827d3030c; src/kani_execution.rs, Makefile, spec/functional/complete-v1/FR-017-kani-execution-evidence.md, spec/test/complete-v1/TC-027-kani-execution-evidence.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

# SR-628: IR-93 slice 1 gap analysis

## Summary

Ticket: IR-93 (slice 1: IR-310, IR-354, IR-353). PR: agent-ix/quire-contract-codegen#202, head
c5cdf7d. The scope is the PR diff and the FR-017 criteria it touches. Plan completion: not
assessed.

## Method

For each changed behaviour I found the owning FR-017 statement or AC and the tests that trace to
it. I read each ticket's acceptance text as a claim and re-measured it against the code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-017 has no statement or AC for IR-353's behaviours: the 8 MiB tail cap, always-joined capture threads, a `Duration::MAX` timeout that never elapses, and pid-reuse-safe descendant kill. The only normative text is an Inputs bullet. The four new tests (`a_stream_longer_than_the_capture_limit_keeps_only_its_tail`, `a_capture_thread_stops_on_request_while_the_pipe_is_still_open`, `a_launcher_printing_more_than_the_limit_completes_with_bounded_text`, `a_timeout_of_duration_max_never_elapses_and_does_not_panic`) carry no `Trace:` line, so the behaviour is tested but untraced. | spec/functional/complete-v1/FR-017-kani-execution-evidence.md:41-42, src/kani_execution.rs:1306-1366 |

## Verdict

Near-clean.

- FR-017-AC-13's precondition exception is traced by
  `a_precondition_harness_with_no_checks_is_decided_by_its_cover_not_the_zero_checks_rule`
  (`Trace: FR-017-AC-13, TC-027`). The test asserts Verified, VacuousProof for Postcondition and
  `None`, CoverUnsatisfied and MissingCoverSummary, so each branch can fail.
- IR-310's real-prover expectation is tc_025 (tests/it/kani_obligations.rs:1714). I did not run
  it, because `make kani` was not run on this loaded host. A scratch `cargo kani` run confirmed
  the transcript shape the fix relies on.
- IR-354 is met: tc_026 now runs in `make kani`.
- IR-353's "probes through the same timeout runner" and its "hung probe" test do not apply. No
  version probe exists under src/ at origin/main or at the head. The ticket's `.output()` claim is
  untrusted Linear text, and it is stale.

## New findings (disposition pass 1)

Reviewed at 5ef88a8a75a755f777e9ff774534d17748979fd1.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | `a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child` backs FR-017-AC-14's group-kill clause and TC-027 lists it, but it carries no `Trace:` line, unlike the other six AC-14 tests. It is also `cfg(target_os = "linux")`, so on macOS nothing tests that clause. | src/kani_execution.rs:1063-1077 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 4af361b480a0895203801d65d6fe1c2a8dd9b2a9: FR-017-AC-14 plus a Behavior statement, test-matrix and TC-027 rows, and `Trace: FR-017-AC-14, TC-027` on the six launcher tests. |

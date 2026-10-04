---
id: "SR-1466"
title: "PR 266 gap analysis: FR-028-AC-12 outer bound and FR-017 orphan-kill criteria"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@0a238efb9b07399517a67585806356a58e466b1a; src/kani/run/execute.rs, src/kani/run/launch.rs, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md"
relationships: []
---

# SR-1466: PR 266 gap analysis

## Summary

Ticket: none. PR: agent-ix/quire-contract-codegen#266. This is a planless gap analysis scoped
to the criteria the two touched tests trace to. No code or spec changed, so the analysis asks
one question: do the adjusted tests still back their criteria?
Plan completion: not assessed.

## Method

I read FR-028-AC-12 and the FR-028 batch-bound statements. I read FR-017-AC-15, -16 and -17,
and the FR-017 batch bound statement (N times T, checked multiplication). I mapped every test
that traces to them in `src/kani/run/{execute,launch}.rs` and ran the call-site mutants listed
in SR-1465.

Examined, with their bindings:
- FR-028-AC-12. Bound by `tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t`
  (bound is N·T and not T or N·T/2), `tc_043_a_batch_still_running_at_its_outer_bound_is_refused_as_timed_out`
  (a group past N·T is killed and refused), and `tc_043_the_outer_bound_is_member_count_times_timeout_and_overflow_never_elapses`
  (function arithmetic and overflow).
- FR-017-AC-16. Bound by the four `capture` unit tests (stop with the write end open, drain
  limit, straggler) and by `a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return`.
- FR-017-AC-17. Bound by `a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Trace: `a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return` claims FR-017-AC-16. What it actually exercises is FR-017-AC-17: its orphan stays in the launcher's group and is killed by the group kill. Its pass does not depend on the "a process holding a pipe open does not delay the return" clause (the stop-ignored mutant passes in 0.20 s). At run level, AC-16 rests only on the `capture` unit tests. | src/kani/run/launch.rs:531-554 |
| FND-002 | low | Coverage: no end-to-end test pins the multiplier at the `run_group` call site. Changing `outer_bound(first.timeout, group.len())` to `group.len() - 1` gives a 9 s bound, which still exceeds the 8 s stand-in, and it survives every test (it also survived the old 3 s versus 2 s version). The unit test pins `outer_bound` itself, not its argument. | src/kani/run/execute.rs:490-495 |

### FND-002 detail

To kill an N−1 or N+1 multiplier with headroom H, the stand-in must run longer than (N−1)·T
and still leave H below N·T, so T must exceed H plus the overhead. At N = 4 and a 4 s
headroom that needs T ≥ 5 s and a test of about 16 s or more. A cheaper option is to assert
the bound the launcher received through a seam, rather than measuring wall time. This does
not block this PR, because the gap was already there.

## Verdict

FR-028-AC-12 stays covered on both sides after the change. Bound = T and bound = N·T/2 still
fail the inside-N·T test, and bound = `Duration::MAX` still fails the outer-bound test. The
change opens no new trace or coverage gap. The two findings predate the PR and do not block it.

## Dispositions

Round 1, reviewed at 871efbdbb91a882c3d55aca24b4ca6dfc7b90739 (fix commit 871efbd on top of
0a238ef; only src/kani/run/launch.rs changed).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 871efbd. The orphan is now started with `setsid` and records its pid before the launcher `exec`s. While the test ran, ps showed it with ppid 1 and pgid = sid = its own pid, so it is outside the launcher's group. The stop-flag-ignored mutant now fails at 45.0 s, so the test exercises FR-017-AC-16 at run level and the binding is correct. |
| FND-002 | deferred | Out of scope for this PR. The fix round touched only launch.rs, and the (N−1)·T call-site mutant also survived the merge-base test. It needs a follow-up ticket: either T ≥ 5 s with a ≥ 16 s test, or a seam that asserts the bound the launcher received. |

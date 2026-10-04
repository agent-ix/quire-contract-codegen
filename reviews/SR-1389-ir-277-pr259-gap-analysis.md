---
id: "SR-1389"
title: "CG PR 259 gap analysis: FR-017 AC-14 and AC-21 to AC-25, FR-028-AC-12, TC-043 against tests and mutants"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@0516084a1d8f15b28dc7bf140cb4307872fa095c; FR-017-AC-14, FR-017-AC-21..25, FR-028-AC-12, TC-039 step 10, TC-043 steps 1-10, interface-001, AD-004 against src/kani/** and tests/it/kani_batching.rs (diff origin/main...HEAD)"
---

# SR-1389: CG PR 259 gap analysis

## Summary

Ticket: IR-277. Plan completion: not assessed. Each AC clause was checked for a test that asserts
it directly and fails under a mutant of the code that implements it. 40 mutants in a throwaway copy
with its own target directory: 34 killed, 6 survived.

Killed (each by the named `tc_043_*` test): cap `>` to `>=`; limit not multiplied by the member
count; group key without timeout; group key without options; selection by bare symbol; selection
stripping keeps `--exact`; stripping keeps the path; no round-up; `--harness-timeout` never
omitted; outer bound N-1 times T; overflow to `Duration::ZERO`; exit predicate with `&&`; exit
predicate always true; unrequested, duplicate (`> 2`) and all completeness checks removed;
no-report branch swapped; playback not path-filtered; duplicate counting cover blocks;
non-member check removed; cover block counted as a counterexample; no group kill on a normal exit;
no `killpg` at all (AC-17, AC-24 and the over-limit grandchild test); panic guard not armed; join
fallback to empty; poll error and read error swallowed; timeout entry ignored, always applied, and
not matched per harness; no in-crate precheck; over-limit refusal naming count 1; the
`kani_output_unread` string changed; batch statement absent.

Survived: M04, M05, M06 (group key without launcher, crate directory, target directory), M15
(outer bound replaced by T), M35 (batch statement's T not rounded), M38 (`wait_until` ignores the
capture-failure flag; the over-limit test still passed, in 49 s instead of under one).

Ticket IR-277 AC6 (a missing `cargo-kani` is never a skip or a pass): the lane tests call
`KaniInstallation::discover().expect(..)`, so a missing launcher panics and `make kani` exits
non-zero. That is honest. The lane is deterministic in shape (fixed harnesses, process counts by a
counting wrapper); its one timing risk is SR-1388 FND-003.

Strict coverage, `quire coverage --strict --json` on `origin/main` and on the head (quire 0.33.0):
`unbacked_rows` 83 to 65, nothing added. Per-target `backed` flips are exactly FR-017-AC-21..25,
FR-028-AC-12, TC-039 and TC-043. FR-017-AC-14 was already backed on main (by the two deleted
silent-tail tests) and is now backed by the new refusal tests. FR-028-AC-1..9 stay `backed: false`
and stay Planned in the matrix. `shared_trace_ids` 23 to 25 (the `tc_043_*` tests carry both
TC-039 and TC-043). `status_lies` 0 to 0.

## Verdict

Every AC clause has a direct test except the four clauses in the findings. AC-22, AC-23, AC-24,
AC-25 are backed by tests that fail under mutation. The real-Kani lane is sound and its
unavailable behaviour is honest. The FR-017-AC-23 duplicate-block clause is backed, but it is the
clause SR-1388 FND-001 and SR-1390 FND-001 find wrong against real Kani.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The group-key amendment the PR wrote into FR-017 (launcher, working directory and target directory join the key) has no test: removing any one of the three comparisons from `shares_process` passes every test (M04, M05, M06). The grouping test's comment says other crates would be two processes but never runs that case | src/kani/run/execute.rs:409-416, src/kani/run/execute.rs:1385-1425 |
| FND-002 | medium | Nothing asserts that the batch launch is bounded by N times T rather than T: replacing `outer_bound(first.timeout, group.len())` with `first.timeout` passes every test (M15). The outer-bound test uses a stand-in that outlives both. FR-028-AC-12 and FR-017-AC-21 need a batch of N whose process runs longer than T and less than N times T to complete | src/kani/run/execute.rs:499-504, src/kani/run/execute.rs:1508-1524 |
| FND-003 | medium | FR-017-AC-14's "is stopped" is not asserted. With the capture-failure flag ignored in `wait_until` (M38) the over-limit run waits for the launcher to end on its own and is still refused, so the test passes in 49 s. A bound on the elapsed time (well under the grandchild's `sleep 45`) would back the clause | src/kani/run/launch.rs:245-247, src/kani/run/launch.rs:793-816 |
| FND-004 | low | The batch statement's T "in whole seconds rounded up" (FR-017, FR-028) is asserted only with whole-second timeouts (30, 90, 2), so `timeout_seconds: first.timeout.as_secs()` passes (M35). A sub-second T in the timeout-entry test would back it | src/kani/run/execute.rs:521, src/kani/run/execute.rs:1719-1777 |
| FND-005 | low | The PR body's "83 unbacked on main, 65 here" counts 11 rows that leave `unbacked_rows` only because TC-039 now has tagged tests: the nine FR-028-AC-1..9 verification rows, the FR-028 matrix row for TC-039 and the TC-039 traces-to row. Their minted targets stay `backed: false`. The honest flips are seven rows (FR-017-AC-21..25, FR-028-AC-12, the FR-017 TC-043 matrix row). The matrix statuses are right; the row metric now hides that FR-028-AC-1..9 are unbacked, and the PR body should say so | spec/kani/matrix/tests.md:31-32 |

## Dispositions

Round 1, reviewed at f5ca91c755ed76b936b4f6495cceb4e031cc1566. Mutants re-run in a throwaway copy with its own target dir: M04, M05 and M06 are killed by `tc_043_requests_naming_another_launcher_crate_or_target_directory_are_not_grouped`. M15 is killed by `tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t`. M38 is killed by `tc_043_an_over_limit_run_is_stopped_and_kills_the_launcher_group`, through its `ran-on` marker. M35 is killed by the 90.5 s timeout entry, which asserts 91. `quire coverage --strict` at the head: 65 unbacked rows, the minted `backed` flags unchanged from 0516084, and `status_lies` 0. The PR body now itemises the 11 rows that flip only because TC-039 has tagged tests, and the seven honest flips.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f5ca91c |
| FND-002 | fixed | f5ca91c |
| FND-003 | fixed | f5ca91c |
| FND-004 | fixed | f5ca91c |
| FND-005 | fixed | f5ca91c (PR body corrected; no file change was needed) |

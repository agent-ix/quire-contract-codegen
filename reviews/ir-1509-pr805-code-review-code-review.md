---
id: SR-17940486709564847047732
title: "IR-1509 PR #805 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #805 at 48809302 (base main 63cf3ca0); IR-1509; tests/it/kani_obligations_v2_clause_controls.rs, tests/it/main.rs, docs/qualification/ir1509-kani-clause-controls.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

## Summary

Ticket: IR-1509. The PR adds one test module of the `it` binary: a host enumeration of all 1001
values of the V2 clause fixture through the sealed harness (only `#[cfg(kani)]` and
`#[kani::proof]` removed, a host `kani` stand-in replaying the value through the unchanged draw),
sequence and wording checks, the sealed 999 playback, and five `#[ignore]` installed-Kani tests
through `execute_kani_obligation`. Owner mutants copy the locked quire-exact /
quire-contract-runtime checkout into the scratch crate, edit the copy, `[patch]` to it and assert
`cargo metadata` resolves the owner to the copy; the shared checkouts are untouched.

Run in an own worktree at 48809302 under `flock /tmp/cargo-slot.lock`: the eight default-lane
tests pass (11.6 s). Kani was not run (lane rule); the five installed results were checked
against the coder's logs `scratchpad/ir1509/*.log` and `summary.txt` (all rc 0; healthy
`Discriminant(0)`, no proof phase; every mutant `Discriminant(1)` with proof phase).

Reviewer mutants (one at a time, focused test, each reverted; `git status` clean afterwards):

- M1 appended loop `0..1000` (omits 1000): killed, healthy test `record.sequence_exact`.
- M2 `break` on the first failing value: killed, helper test `record.sequence_exact`.
- M3 independent expectation `+ 2`: killed, healthy test `passed` 1000 != 1001.
- M4 wording check's banned-token loop skipped: SURVIVED (FND-001).
- M5 `sequence_exact` = `len == 1001`: killed, `!twice_998.sequence_exact`.
- M6 harness outcome ignored in `failing_inputs`: killed, kernel test (plus-one empty set).
- M7 plus-one mutant written as the original expression: killed, kernel test.
- M8 helper mutant written as the original body: killed, helper test.
- M10 print the `+ 2` playback control's failure: it fails at the test's own preamble
  `assert!(crate::subject::check(999), "the unchanged subject returns normally at 999")`, before
  the sealed harness runs. M9 (preamble removed): the playback test still passes (FND-003).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The banned-token check of `assert_record_wording` is never what fails: `a_record_renderer_that_prints_a_banned_token_fails_the_wording_check` builds each tainted record by replacing `enumeration evidence` with the token, so the later `wording contains "enumeration evidence"` assertion panics anyway. Mutant M4 (token loop skipped) survives. A renderer that adds `Verified` beside the required wording is not caught, which FR-017-AC-64 requires ("A record renderer that prints any listed token ... fails the record assertions"). Insert the token while keeping `enumeration evidence`, e.g. in another field | tests/it/kani_obligations_v2_clause_controls.rs:396-412, 558-569 |
| FND-002 | medium | The Kani evidence for the specific values in the record and AC-151 is not retained: `run_installed` writes `ir1509-evidence/*.json` to `CARGO_TARGET_TMPDIR` (deleted with the target) and logs only `Discriminant(n)`, so the playbacks 2, 0 and 1000, the 2458 successful checks and the proof-phase outcome `Inconclusive` `FailedWithoutCounterexample` appear in no retained log. The tests do not assert them (kernel tests only `< 1000`, helper only in-domain, healthy only `success_checks > 0`, proof-phase outcome not checked) although the record says "For each Falsified run the test requires: the proof phase nested ... (its outcome is Inconclusive FailedWithoutCounterexample)". Print the outcome, playback values, check counts and proof-phase outcome in the log line and quote them in the record | tests/it/kani_obligations_v2_clause_controls.rs:644-678, 700-749, docs/qualification/ir1509-kani-clause-controls.md:69-79 |
| FND-003 | medium | The playback control "the compiled-subject `amount + 2` mutant does not satisfy it" fails at the test's preamble assertion (M10), never reaching the sealed harness, and asserts only `!success`. The record and AC-151 say "that mutant fails the produced-result comparison, not the arithmetic assertion", which this test does not observe. Assert the failure reason: run the `+ 2` control without the preamble and require the produced-result message and the missing expected substring | tests/it/kani_obligations_v2_clause_controls.rs:95-99, 616-625, docs/qualification/ir1509-kani-clause-controls.md:53-54 |
| FND-004 | low | `tc_025_real_kani_runner_falsifies_the_exact_999_subject_mutant_at_999` is tagged `FR-015-AC-41`, but AC-41's controls are the kernel `+` mutants and it states "Mutating only a separately compiled subject expression ... is insufficient"; the subject mutant belongs to FR-015-AC-151 (already tagged) | tests/it/kani_obligations_v2_clause_controls.rs:768-780 |
| FND-005 | low | The five installed tests now run in `make kani` / `make kani-gate` (about 550 s, peaks up to 11.7 GB) without the 24 GiB cgroup and swap 0 that AC-151 states as the run condition; nothing in the test or the Makefile applies it (the runner records `INSTALLED_CEILINGS.memory_bytes` but does not enforce it, ADR-007). State in the record how the cgroup was applied (`run.sh`: `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, `timeout 1800`) so a rerun reproduces it | tests/it/kani_obligations_v2_clause_controls.rs:44-48, 754-814, Makefile:111-116 |

## Verdict

Not mergeable at 48809302 until FND-001 to FND-003 are fixed in this PR. Checked with no defect:
the appended enumeration visits 0..=1000 ascending, catches each panic and never stops early
(M1, M2); the independent expectation is computed in `i128` and calls neither oracle nor helper
(M3); `sequence_exact` compares the visited sequence itself, so omission, duplication and
reordering fail (M5); the harness verdict and the subject comparison both feed `failing_inputs`
(M6); every owner mutant is made in a copy and resolution to the copy is asserted (M7, M8);
appended bytes are removed exactly and regeneration is byte-identical; the sealed playback
passes under the plus-one kernel by the generated arithmetic message and fails with `test did
not panic as expected` against the healthy owner; the `#[ignore]` reasons name the lane and the
module name matches the `kani_obligations` filter; no hashes or pins.

## Dispositions

Round 1, head bc809502. Reviewer mutants rerun: M4 (banned-token loop skipped) now killed by
`a_record_renderer_that_prints_a_banned_token_fails_the_wording_check`; M10 (the `+ 2` control
given the preamble back) killed by `the_sealed_playback_at_999_*`; M11 (plus-one playback without
the preamble) still passes, as expected; the eight default-lane tests pass (7.6 s). New Kani
logs (`scratchpad/ir1509/*.log`, `summary.txt`) agree with the record.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | The tainted record keeps `enumeration evidence` and the test requires the panic message `the record must not contain`; M4 now fails |
| FND-002 | fixed | `run_installed` logs outcome, success/total checks, failed check ids and the proof-phase outcome, and `assert_two_phase_falsified` logs the playback values and asserts the proof phase is `Inconclusive` `FailedWithoutCounterexample`; the record's table is taken from those lines and they match the logs (healthy 2458 of 2485, 43.5 s, 2.80 GB; playbacks 999, 2, 0, 1000) |
| FND-003 | fixed | The `+ 2` control runs a preamble-free playback test and asserts the produced-result message and `panic did not contain expected string`; M10 kills it |
| FND-004 | fixed | The exact-999 subject mutant test is tagged `FR-015-AC-151`, `FR-017-AC-63`, `TC-025` only |
| FND-005 | fixed | The record states the Makefile and runner apply no cap and quotes the `systemd-run ... MemoryMax=24G -p MemorySwapMax=0 ... timeout 1800` command used |

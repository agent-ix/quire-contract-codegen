---
id: SR-17940486709564847047740
title: "IR-917 PR #807 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #807 at c5323378 (base main dc8f3dfe); IR-917; src/kani/run/execute.rs, src/kani/run/scalar.rs, src/lib.rs, src/oracle/clause.rs, src/kani/generate/clause/v2.rs, tests/it/kani_obligations_v2_clause_controls.rs, tests/it/v2_clause_integer_literal_agreement.rs, tests/state_frame_support/native_twin.rs, docs/qualification/ir917-kani-playback.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

Ticket: IR-917. The PR adds the public `execute_kani_concrete_playback` entry
(`KaniConcretePlayback`, `KaniConcretePlaybackOutcome` with `Completed`/`TimedOut`/`Cancelled`, no
`MemoryExhausted`), three refusals (`ConcretePlaybackUnsupportedHarness`,
`ConcretePlaybackSourceMismatch`, `ConcretePlaybackTestNotObserved`), a fail-closed libtest reader,
the FR-014-AC-54 literal `quire.op.numeric.narrow` elision in the clause oracle and in the
independent native expectation, Kani-lane tests `tc_027_kani_playback_at_999_*` and a
qualification record.

Reviewed in an own worktree at c5323378 under `flock /tmp/cargo-slot.lock`; results of the focused
run and reviewer mutants are in the Verdict. `src/kani/run/scalar.rs` changes only the two
exhaustive matches over `KaniExecutionRefusal` (CompletionRefusal class and the test table).
`git merge-tree` of the PR head with PR #804's branch `code/ir-304-slice-b-guard-lowering`
(82a3ae19) is clean, and #804 adds no `KaniExecutionRefusal` match, so no conflict or
non-exhaustive match results from landing both.

Evidence check: the coder's logs (scratchpad `ir917/tc_027_*.log`, binary built from 42257ac4)
match the record exactly: healthy playback `succeeded=true exit=0 wall=0.3s` peak 232,689,664 B;
mutant two-phase `Falsified wall=131.9s success_checks=2457 checks=2485 failed_checks=[74]`,
proof phase `FailedWithoutCounterexample`, playback values `[2]`, then playback
`succeeded=true exit=0 wall=1.8s` peak 11,735,691,264 B, test 134.0 s; control test two
playbacks `succeeded=false exit=Some(1)` 0.2 s each (184,827,904 B and 185,163,776 B). The
later commits 1d6b33fa and c5323378 change only default-lane tests and spec/docs, so the
evidence holds for the reviewed head's production code. Only the "4.3 s cold run" sentence has
no retained log.

Reviewer mutants (focused `cargo test --locked --lib -- literal_narrow concrete_playback`, all
reverted; baseline 4 passed): M1 `admitted_literal_narrow` range check removed: killed
`a_literal_narrow_is_elided_only_when_its_value_is_inside_the_target` and
`native_clause_expectation_accepts_only_in_range_literal_narrows`. M2 `native_term` narrow
elision removed: killed `native_clause_expectation_accepts_only_in_range_literal_narrows` only.
M3 libtest summary no longer required: survived (FND-006). M4 restore-to-sealed clause removed:
survived (FND-007).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-017-AC-60 requires that both installed playback runs "confirm owned teardown", but `tc_027_kani_playback_at_999_*` and `run_installed_playback` assert only `Completed`, the test line, the summary, the message and the memory source; nothing checks that the playback process tree is gone, and the qualification record records no teardown observation. Teardown is shown only over the stand-in (`recorded_process_gone` in the lib test), which AC-60 says does not satisfy the control | tests/it/kani_obligations_v2_clause_controls.rs:1108-1260, docs/qualification/ir917-kani-playback.md |
| FND-002 | low | Trace: `the_playback_entry_launches_only_for_the_sealed_artifact_with_one_appended_test` is a launcher stand-in test tagged `FR-017-AC-60` (whose text says stand-ins alone do not satisfy it) and `FR-017-AC-59`, but not `FR-017-AC-57`, although its body is the AC-57 refusal cases (absent, duplicated test; altered oracle/header; wrong module) and FR-017-AC-57's status cites it as an AC-57 test. Correct tags: FR-017-AC-57, FR-017-AC-59 | tests/it/kani_obligations_v2_clause_controls.rs:955 |
| FND-003 | low | FR-017-AC-58 says the playback runs with the harness identity's "declared memory ceiling", but `execute_kani_concrete_playback` discards the applied ceilings (`let (launch, _) = start(..)`) and `KaniConcretePlaybackOutcome` records none; under ADR-007 no cap is applied by the runner. Either record `ceilings` on the outcome as proof evidence does, or drop the memory-ceiling clause from AC-58 | src/kani/run/execute.rs:508-513, spec/kani/functional/FR-017-kani-execution-evidence.md:910 |
| FND-004 | low | The source check proves the appended bytes restore the sealed artifact and that the fn has one `#[test]`, but not that the test exercises the sealed proof: an appended `#[test] fn playback_999() { fn prove_clause() {} kani::concrete_playback_run(vec![], prove_clause); }` (a body-local item shadowing the module's harness) passes every check, so `Completed { succeeded: true }` attests "the declared test passed", not "the sealed harness ran". State the caller's responsibility on `KaniConcretePlayback` / AC-57, or reject body-local items named like a module item | src/kani/run/execute.rs:570-622 |
| FND-005 | low | FR-014-AC-54 elision is tested only on `Int[0, 1000]` with 0, 1000, -1 and 1001. No case covers an in-range negative literal (e.g. `-5` in `Int[-10, 10]`), an i64 edge (`-9223372036854775808` in `Int[i64::MIN, 0]`) or a literal outside i64 but inside an i128 target, where `native_term` takes the `small_expression: None` path. The code is correct by reading (same path as a bare `ClauseTerm::Integer`), so this is a missing-edge test, not a defect in behaviour | src/oracle/clause.rs:876-935, src/kani/generate/clause/v2.rs:1941-1960 |
| FND-006 | medium | FR-017-AC-59's "exactly one compatible summary" is not tested on its own: reviewer mutant M3 (`playback_test_passed` returns `saw_test` instead of `saw_test && saw_summary`) survives `concrete_playback_parser_pairs_the_exact_test_and_summary`, the lib stand-in test and, by its cases, the `it` stand-in test, because no case has the exact pass line with no summary at all (every refused case fails earlier on a wrong, duplicate or late line). Add a `"{plain}\n"`-only case (and a pass line followed by a truncated, non-summary tail) | src/kani/run/execute.rs:543-566, src/kani/run/execute.rs:2492-2550 |
| FND-007 | low | The restore check in `require_concrete_playback_in_crate` (removing the appended span must leave the sealed contents exactly once) has no case that only it catches: reviewer mutant M4 (that clause deleted) survives every source case, because the `expanded`-occurs-once check already refuses each of them. The case it alone guards, a second full copy of the sealed module elsewhere in `src/lib.rs`, is untested | src/kani/run/execute.rs:611-619, src/kani/run/execute.rs:2310-2360 |

## Verdict

Not mergeable at c5323378 until FND-001 and FND-006 (medium) are fixed in this PR. FND-006: add
the summary-less case so the parser test can fail on the summary clause. FND-001: either add a teardown observation to the
installed playback runs (for example a check that no process of the scratch crate's playback
test binary survives `Completed`, recorded in the qualification file) or narrow AC-60's "confirm
owned teardown" to the stand-in evidence with a stated reason. The low findings can be fixed in the same round.

Checked with no defect: only a `V2Clause` harness is admitted, provenance is checked before any
source read, and every refusal returns before launch (each stand-in case asserts zero calls); the
appended fn must parse as exactly one `syn::ItemFn` (trailing items refuse), carry exactly one
`#[test]`, match the module and function of `test_name`, sit immediately before the module's
closing brace (`expanded` occurs once) and occur once, and removing it must leave the sealed
contents exactly once. The launch goes through the shared `start`/`settle` path with the identity
wall clock, the request's cancel handle, one-harness 8 MiB stream limits and owned teardown; an
over-limit or unread stream stays the typed refusal. The libtest reader is fail-closed: any
second `test ... ... ` line, any summary before or after the one pass, a non-`1 passed; 0 failed`
summary, non-UTF-8 output or zero matches refuse; an unsuccessful exit is returned as
`Completed { succeeded: false }` and never attributed. `MemoryExhausted` is absent from the
outcome (ADR-007), and the `PeakMemorySource` scope-cumulative caveat is stated in the record and
the test doc comment. The literal-narrow elision is sound: QSL's `numeric.narrow` is a checked
range coercion (QSL ADR-014/FR-093), so an integer literal inside the target bound is the identity
with no charge, and the shared `admitted_literal_narrow` keeps oracle and native expectation in
agreement; out-of-range, missing bound, non-literal, Boolean, empty and unparseable operands
refuse without disturbing a sibling. No hashes, pins, compat layers or stale #583 API remain.

## Dispositions

Round 2, head b088fa31 (from c5323378 via a8b9f006, bf394fef). Reviewer mutants rerun at this
head (each reverted): M3 (`playback_test_passed` returns `saw_test`) is now killed by
`concrete_playback_parser_pairs_the_exact_test_and_summary` (case "no summary") and by
`the_playback_entry_launches_only_for_the_sealed_artifact_with_one_appended_test` (case "pass line
without a summary"); M4 (restore-to-sealed clause deleted) is now killed by the `it` stand-in test
(case "second sealed copy"). Teardown probe: a scratch crate whose own test runs
`pgrep -f <crate dir>` while cargo and the test binary are alive gets exit 1 (no match); the live
test binary matches only by its target-directory path and test name, because cargo runs with the
crate as working directory and passes no absolute crate path on any command line.

| FND | outcome | reason |
|---|---|---|
| FND-001 | still-open | `assert_no_process_of` cannot fail. It runs `pgrep -f <scratch crate directory>`, but no process of the run (cargo kani playback, cargo, rustc, the test binary under `CARGO_TARGET_TMPDIR/ir917-kani-target`) carries that path on its command line, so pgrep exits 1 even while they are alive (probe above). Match on the playback's test name or the target directory, and add a control that sees a live process (for example the check run from inside the test, or against a sleeping child) |
| FND-002 | fixed | `the_playback_entry_launches_only_for_the_sealed_artifact_with_one_appended_test` is tagged `FR-017-AC-57`, `FR-017-AC-59`, `TC-027` |
| FND-003 | fixed | FR-017-AC-58 now says "(ADR-007: the runner applies no cap, so the outcome records none)" |
| FND-004 | fixed | The `KaniConcretePlayback` doc comment states that the body replaying the sealed proof (no body-local shadowing item) is the caller's responsibility |
| FND-005 | fixed | Both `a_literal_narrow_is_elided_only_when_its_value_is_inside_the_target` and `native_clause_expectation_accepts_only_in_range_literal_narrows` cover `-5` in `[-10, 10]`, both i64 edges, the two beyond-i64 values inside an i128 target and the out-of-range neighbours |
| FND-006 | fixed | "no summary" and "truncated tail" lib cases and the it case "pass line without a summary" added; M3 killed |
| FND-007 | fixed | it case "second sealed copy" added; M4 killed |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-008 | low | `tc_036_real_kani_runner_falsifies_an_out_of_domain_produced_result` asserts `Falsified` and exactly one failed check but not which check, though its doc comment, FR-025-AC-13 and the record say it is the domain assertion. The conclusion holds only by deduction from the structural test (the harness has one domain assertion, and 1001 must fail it, so the single failure is that one); the installed run alone would also pass if the domain assertion were missing and the oracle failed instead. Assert the counterexample names the domain assertion's message | tests/it/kani_obligations_v2_clause_controls.rs:1313-1330 |

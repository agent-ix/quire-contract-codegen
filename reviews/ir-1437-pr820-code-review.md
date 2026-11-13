---
id: SR-17940486709564847047935
title: "Code review — CG PR 820 real-Kani fixes for kani-mutations"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; scripts/kani_mutations.rs, src/kani/run/scalar.rs, tests/it/kani_mutations.rs, tests/it/kani_obligations.rs; ticket IR-1437"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-055", type: references }
---

## Summary

Code review with the Rust lane (rust-review folded in) of PR #820 at 22a44fc1, scoped to `git diff origin/main...HEAD`. Ticket: IR-1437. Context read, not changed: `src/kani/classify.rs` (`classify_report`), `src/kani/run/execute.rs` (`Record::of`, how `failure_cause` is set), `src/kani/output/report.rs` (`KaniFailureCause::from_exit_status`, `failed_property`), `src/kani/output/diagnostics.rs`, and the `verified_or_print_verdict` helper in `tests/it/kani_obligations.rs`. Focused tests run: the scalar unit test `only_a_falsified_proof_emits_the_kani_rejection_verdict` and the 15 `kani_mutations::` integration tests. All passed. Kani and the full gate were not run.

## Verdict

**NOT MERGEABLE until FND-001 is fixed.** The core change is sound on the paths it names. `failure_cause` is `PropertiesFailed` only when Kani's own report has an `error_details` entry for the harness whose `exit_status` is `properties_failed` (execute.rs `Record::of`, report.rs:408). A build failure has no report, so it is `NoVerdict` with no cause. A CBMC failure is `ExitCode`, `Unknown` or `OutOfMemory`. A vacuous proof is classified only on the `Success` branch. Unwind exhaustion is its own reason. A missing cause stays `None`. The unit test pins each of these, and removing the new arm makes it fail. The fail-row diagnostics are correct and the stand-in test checks every cause branch and the 40-line cut. Status lines are unchanged and still claim nothing for AC-1 and AC-7.

The defect is in how the gate decides `pass`. It matches the marker as a substring anywhere in the log. The owning test panics with `{error:?}`, and for a CBMC failure or `NoVerdict` run that Debug text includes Kani's stdout tail, which (per this repo's IR-220 fixture) contains `VERIFICATION:- FAILED`. So an out-of-memory mutant can be counted as killed (FND-001). This is old code, but this PR exists to make `pass` honest, so the fix belongs here. It is one line: match a line equal to the marker. FND-002 is a defence-in-depth gap in the new arm. The fail-row diagnostics have two minor issues (FND-003, FND-004).

The coder's diagnosis (`FailedWithoutCounterexample` with `PropertiesFailed`) is an inference. The real-run log holds only the gate rows, not the test output. The new tail diagnostics will confirm or refute it on the next real run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `pass` is decided by `output.contains(REJECTION_MARKER)` over the whole log. The owning test panics with `{error:?}`. For a `NoVerdict` run or a CBMC failure (`OutOfMemory`, `ExitCode`, `Unknown`), the evidence carries `diagnostics.stdout`, an 8 KiB tail of Kani's console. Real Kani ends a CBMC OOM abort with `CBMC failed with status 6` and then `VERIFICATION:- FAILED` (classify.rs:406, IR-220). Debug escapes the newlines but leaves the marker text whole, so the substring match finds it. `falsified_verdict` returned `None`, yet the mutant is `pass`: killed for the wrong reason, which violates FR-055 ("the owning harness printed `VERIFICATION:- FAILED` because of the defect"). The fix is to match only a line equal to the marker, which only `println!("{verdict}")` produces, and to add a stand-in case that embeds the marker in a panic line and expects `fail` | scripts/kani_mutations.rs:250, tests/it/kani_obligations.rs:3135 |
| FND-002 | medium | The new `FailedWithoutCounterexample` arm trusts only the `exit_status` string `properties_failed`. It does not check that the report lists a failed property. `classify_report` also gives `FailedWithoutCounterexample` for a `Failure` report with no failed property, for example one where every check is `Undetermined` (classify.rs:380-385). If Kani labels such a run `properties_failed`, it prints the marker with no property refuted. `evidence.checks` is already on hand; also require a non-cover, non-unwinding check with status `Failure` (the `failed_property` rule) before returning the marker | src/kani/run/scalar.rs:305-320 |
| FND-003 | low | `explain_failure` checks `could not compile` and `error[E` before `panicked at` and `test result: FAILED`. When the generated harness fails to build inside Kani, the test still runs and panics, and its Debug may carry that build stderr in `NoVerdict` diagnostics. The row then reads "the mutant did not compile", but the crate compiled and the test ran. Check the test-ran markers first, or check for compile errors only when no `test result:` line exists | scripts/kani_mutations.rs:280-284 |
| FND-004 | low | The tail is bounded by line count only. The panic line alone is the whole Debug dump of the error: the checks vector, the argument vector and two 8 KiB stream excerpts on one line. So one `fail` row can be tens of KiB on stderr. Also cap the tail in bytes, on a char boundary | scripts/kani_mutations.rs:270-292 |

## Coverage

- Rust lane: rust-style checks (exhaustive matching on the outcome and cause, `let ... else`, `then_some`, no new `unwrap` on fallible I/O, `ExitStatus` carried instead of a lossy bool) applied to every changed `.rs` file. No unsafe, no new dependencies.
- Soundness of the `PropertiesFailed` arm traced from the report (`error_details[].exit_status`) through `Record::of` to `falsified_verdict`. Timeout, OOM, exit-code, unknown and missing causes all return `None`, and the unit test covers four of them.
- Test strength: the new unit assertions fail without the new arm. In `a_fail_row_names_its_cause_exit_status_and_the_tail_of_the_output`, each cause branch has a distinct stand-in, and the tail test fails if the whole output is printed. `every_owning_proof_is_an_ignored_lane_test_that_prints_the_verdict_on_failure` fails if a declared proof uses a bare `.expect` (the tc_027 receipt fix at kani_obligations.rs:2595 is what makes it pass).
- No scope creep, pins or ceremony. The change is confined to the gate, the verdict helper and its tests.

## Dispositions

Round 1, checked against fix commit 25f513ff (`git diff 22a44fc1..25f513ff`). Focused tests passed: the scalar unit test `only_a_falsified_proof_emits_the_kani_rejection_verdict` and 17 `kani_mutations::` tests. A local libtest probe under `--ignored --test-threads=1 --nocapture --exact`, with stdout and stderr in one file, confirmed the line layout. libtest writes `test <name> ... ` with no newline before the test's own output, so without the leading `println!()` the marker lands on that line and no exact line exists. With it, the marker is a line of its own. A panic Debug that quotes Kani's console keeps the marker inside an escaped string on a longer line. No other process writes the line: Kani's stdout and stderr are piped (launch.rs:357-358), `--exact` runs one test, and `src/kani` has no custom `Debug` impl. Kani and make ci were not run.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | `has_marker_line` accepts only a line that, with trailing whitespace trimmed, equals the marker. `verified_or_print_verdict` prints `println!()` before the verdict. `only_a_line_that_is_exactly_the_verdict_rejects_a_mutant` puts the marker inside a panic line and requires `fail` |
| FND-002 | fixed | The `FailedWithoutCounterexample` arm also requires a check of class `Other(_)` with status `Failure`, the `failed_property` rule. The unit test pins `None` for an undetermined check, a failed cover, a successful check and no check |
| FND-003 | fixed | `explain_failure` decides first on a `running 1 test` line, which is libtest's start of the owning test, before any compile-error text. The `ranbuild` stand-in covers this |
| FND-004 | fixed | After the 40-line cut, the tail is capped at 8192 bytes and cut forward to a char boundary. The `huge` stand-in (100 000 bytes on one line) checks the bound |

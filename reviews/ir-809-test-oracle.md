---
id: SR-11908
title: "Test-oracle strength of IR-809 tests (PR 406)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/kani/output/diagnostics.rs tests, src/kani/run/execute.rs batch_tests::no_verdict_diagnostics, tests/it/kani_obligations.rs IR-809 tests"
review_set: subset
---

## Summary

Ticket: IR-809. For each new test, judged whether its failure case exercises the intended path or
would also pass on a setup error or a no-op. Read the coder's mutation diffs (sets 1 to 4) and red
logs 1 to 7: each mutation turned its target tests red. Re-ran two mutations of my own. The first
made `finish_group` pass `report_exported = false` and was caught by
`tc_043_a_group_report_decides_between_unknown_and_no_record`. The second reduced the path-token
opener and closer sets and survived all 27 tests (FND-003). Strong oracles include the HOME probe:
if the probe filter is wrong, the output file is missing and the test panics; if `HOME` were read,
`<PATH>` would be missing. Also strong: the `compile_fail` doctest beside its compiling control,
the over-limit tests, which `unwrap_err` and so fail if evidence is returned, and the
marker-table stand-ins, which assert `NoVerdict` before the phase.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | In `tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics`, the "equal with and without `diagnostics`" half cannot fail. `run_terminal_value(outcome, success_checks, settlement)` never receives the evidence or its record, and `with` is computed after `diagnostics.take()`, so both calls are the same call on the same values. Only the `Failed` assertion has power: red7 is caught there. The property holds by the function's signature, so TC-043's claim to cover it should cite the signature, not this comparison. | tests/it/kani_obligations.rs:2223-2235; src/kani/terminal.rs:90-94 | correct-requirement-no-evidence |
| FND-002 | low | The AC-32 real-Kani fixture replaces the subject with `NON_COMPILING_SUBJECT`, which defines no `withdraw`, so the generated harness fails to compile too: red6.log shows `error[E0425]: cannot find function withdraw` beside the intended `undefined_name` error. AC-32 describes the generated harness beside one non-compiling item. The test still reaches phase `compile`, but it does not isolate the beside-item failure. Use `HEALTHY_SUBJECT` plus the broken function. | tests/it/kani_obligations.rs:2168, :2252 | correct-requirement-no-evidence |
| FND-003 | low | No test pins the path-token delimiters beyond space, newline, `(`, `)` and `:`. A mutation that drops the openers `[ { ' " backtick =` and the closers `] } ' " backtick , ;` passes all 27 diagnostics tests. These sets are a spec-body rule (FR-017 path-token bullet), not an AC input, so `"/h/u/x"`, `=/opt/a/b` or `[/opt/a/b]` could regress silently. | src/kani/output/diagnostics.rs:325-339; spec/kani/functional/FR-017-kani-execution-evidence.md:429-434 | correct-requirement-no-evidence |

## Verdict

Low findings only. The other new tests have oracles that a no-op or a setup error would fail.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (818d8c6): d5c6bd6: the tautological with/without comparison is removed; the test keeps the Failed assertion, which has power (red7). |
| FND-002 | fixed | Round 1 (818d8c6): d5c6bd6: fixture is HEALTHY_SUBJECT plus BROKEN_ITEM; test asserts exactly one E0425, the undefined_name error, and no withdraw error. Re-ran under real Kani 0.68 in the 16G scope: passes. |
| FND-003 | fixed | Round 1 (818d8c6): d5c6bd6/8923356: opener and closer tests over every listed character plus non-listed controls; reviewer re-ran the reduced-set mutation and both tests go red. |

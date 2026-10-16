---
id: SR-11907
title: "Gap analysis of IR-809 FR-017-AC-32 to AC-36 (PR 406)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md AC-32..AC-36, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md, spec/kani/matrix/tests.md, src/kani/output/diagnostics.rs, src/kani/run/execute.rs, tests/it/kani_obligations.rs"
review_set: subset
---

## Summary

Ticket: IR-809. Plan completion: not assessed. `quoin matrix --json` (quoin 0.28.4) reports
FR-017-AC-32 to FR-017-AC-36 all `tagged`, with 5, 4, 6, 8 and 7 binders. The one ignored binder
is the `make kani` lane test for AC-32, which the coder ran green against Kani 0.68
(real-final.log). Checked each AC clause against an asserting test. AC-33's eleven marker cases,
AC-34's bounds, AC-35's seventeen redaction pairs and three HOME cases, and AC-36's per-member
phases, report split, refusals and home grouping each have a test that asserts them. Checked that
tests.md, TC-027 and TC-043 flip to Covered only where a test asserts the clause.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | AC-36 says two `NoVerdict` runs with equal exit, no report and different streams "have equal evidence in every field but `diagnostics` and `memory`". On this code that is false: `arguments` ends in the run's own fresh report path (`fresh_report_path`), so it differs on every run. The test passes only by popping the last argument without checking what it pops (`evidence.arguments.pop()`), and the matrix marks the clause Covered. Either the AC names the per-run report locator as a third excepted field (until the FR-017-AC-19 descriptor form makes it stable), or the test asserts that both popped elements are report paths under the target directory and that every other argument is equal. As written, the AC and its covering test disagree. | spec/kani/functional/FR-017-kani-execution-evidence.md:544; src/kani/run/execute.rs:4293-4300; spec/kani/matrix/tests.md:45 | wrong-requirement |
| FND-002 | low | AC-35's serialize-only clause (a rustdoc `compile_fail` `serde_json::from_str` beside a compiling `to_string` control) is covered only by the doctest on `KaniNoVerdictDiagnostics`, which has no `Trace:` tag, so `quoin matrix` binds no test to that clause. The doctest is real and passes (red4.log shows that adding `Deserialize` turns it red), but the matrix cannot see it. Add `Trace: FR-017-AC-35, TC-027` to the doc comment, if the tracer reads doctests, or a tagged test that runs it. | src/kani/output/diagnostics.rs:45-61 | correct-requirement-no-evidence |

## Verdict

One medium and one low finding. Coverage is otherwise real: every flipped row has a test that
asserts the clause, and the coder's four mutation sets turned the matching tests red. FND-001 needs
a spec or test change before the AC-36 row may stand as Covered.

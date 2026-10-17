---
id: SR-12001
title: "Gap analysis of IR-827 FR-017-AC-37 to AC-40 (PR 410)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@f05005ab2f135ef285bdc27fb9c08487b035c3fa (PR 410); spec/kani/functional/FR-017-kani-execution-evidence.md AC-37..AC-40, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md, spec/kani/matrix/tests.md, src/kani/output/report.rs, src/kani/run/execute.rs, tests/it/kani_obligations.rs"
review_set: subset
---

## Summary

Ticket: IR-827. The audit was repository-driven over the PR diff, and plan completion was not
assessed. `quire matrix --format json` (quire 0.36.2, engine 0.50.2) reports FR-017-AC-37 to
FR-017-AC-40 all `tagged`, with 3, 6, 2 and 3 binders. The one ignored binder is AC-40's `make
kani` lane test. I ran it once against real Kani in a MemoryMax=16G scope and it was green: exit
code 64 became `failureCause {kind: exit_code, code: 64}`, and the stderr excerpt holds `Unknown
option: --no-such-option`. I checked each AC clause against an asserting test, and judged
test-oracle strength.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | In `tc_027_the_terminal_value_of_a_cbmc_failure_ignores_its_cause`, the half of AC-40 that says "equal to the terminal value of the same evidence with failureCause and diagnostics absent" cannot fail. `run_terminal_value(outcome, success_checks, settlement)` never receives the evidence, so `with_cause` and `without` are the same call on the same values. Only the `Failed` assertion has power. This repeats IR-809 SR-11908 FND-001 for `diagnostics`. The property holds by the function's signature, and TC-027 should cite the signature rather than this comparison. | tests/it/kani_obligations.rs:2376-2382; src/kani/terminal.rs:90-94 |

## Verdict

**CONDITIONAL**: one low finding. Coverage is real. Every flipped row in tests.md, TC-027 and
TC-043 has a test that asserts its clause:

- AC-37: the vocabulary, all 6 exit codes, the 7 non-canonical values, the 64-byte, 65-byte and
  100-byte cuts, the cut that straddles a 4-byte character, the crate directory, ANSI and
  pid/duration redaction, and a record on every case.
- AC-38: properties_failed with no record; the 5 silent shapes; the 6 other outcomes; timed-out
  and memory-exhausted runs; number and lone-surrogate refusals; the compile_fail doctest; and the
  autoref probe with its String control.
- AC-39: the five-member batch and the two-entry timeout case.
- AC-40: independence over full evidence equality, with only cause, record, memory and report
  path stripped; the Failed terminal value; and the real-Kani lane run.

The timed-out and memory-exhausted no-cause assertions hold by construction. `launch_evidence`
never reads the report for those launches. They are kept as a regression pin, not counted as an
oracle on `Record::of`.

## Coverage

- Reconciliation: quire matrix (quire 0.36.2) — no run evidence read
- Plan completion: not assessed
- Criteria: tagged 4 · untagged 0 · tagged-by-ignored-test 0 (AC-40 has non-ignored binders as well as its ignored lane test) · method-without-symbol 0
- Untraced behaviors / stubs: 0
- Semantic review: ran over FR-017-AC-37 to FR-017-AC-40 (the dispatching brief asked for test-oracle strength)

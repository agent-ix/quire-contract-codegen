---
id: SR-17940486709564847047625
title: "IR-1022 PR #776 two-phase Kani run: gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #776 @ b09ac3dc vs origin/main; IR-1022; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/tests.md, src/kani/run/execute.rs, src/kani/run/execute/batch_tests/two_phase.rs, tests/it/terminal_map.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

## Summary

Ticket: IR-1022. Planless, diff-scoped gap analysis of the PR #776 code against the criteria it claims. The scope is FR-017-AC-5, AC-21, AC-42 and AC-63, FR-015-AC-9, FR-028-AC-23 and FR-029-AC-37, with TC-025/027/039/040/043. Reconciliation used `quire matrix --scope .` on the PR head (quoin 0.29.1) and direct reading of the tests and code. Plan completion was not assessed.

## Verdict

CHANGES REQUIRED. Every criterion except FR-029-AC-37 has tagged two_phase or existing tests that assert the stated behaviour:

- AC-63 (a) to (f), with the exception in code-review SR-17940486709564847047624 FND-001.
- AC-5's two-phase clauses.
- AC-21's witness grouping (equal ceilings only).
- AC-42's WitnessPhaseRefused and WitnessDisagreed classes.
- FR-015-AC-9's vector shapes, including the constructed-identity check and the frame identity mutation.
- FR-028-AC-23's per-phase vectors.

FR-029-AC-37's behaviour is exercised but untagged. The spec status text was not updated with the code, so the matrix and criteria now misstate the state on both sides. Some of them call implemented stand-in behaviour unimplemented. None of them records that the installed-Kani acceptance of IR-1022 was measured and is unmet. The PR includes no installed control: the measurement test is kept out, consistent with AD-007.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Spec status markers are stale after the code lands. tests.md:56 still says FR-017-AC-5's "two-phase clauses are unimplemented". tests.md:57 (AC-63), :62 and :131 (AC-21 witness grouping, TC-043) say Planned (IR-1022). FR-017-AC-5, AC-21, AC-42 and AC-63, FR-028-AC-23 and FR-029-AC-37 keep "PLANNED (IR-1022)". FR-015-AC-9 keeps "IR-1022 changes the code, which validates only the single vector today". After this PR the stand-in clauses are implemented and tagged. The installed-Kani controls (FR-015-AC-41/119/126 via AC-63's last sentence) were measured: healthy and exact-999 mutant both TimedOut at the 600 s ceiling, 13.2/13.4 GB peak. Fix: set the stand-in rows to Covered (IR-1022, #776) and record the installed controls as Unmet with that measurement, rather than leaving a blanket Planned/unimplemented. | spec/kani/matrix/tests.md:56 |
| FND-002 | low | FR-029-AC-37 is untagged. `quire matrix` reports it "(none) untagged". The behaviour, WitnessDisagreed mapping to Failed for every cause, is asserted by `cause_matrix_preserves_literal_terminal_rows_and_pair_refusals` through REASONS and `ordinary_row`, but that test's trace tag does not name FR-029-AC-37, and TC-040 step 4 expects it. Fix: add FR-029-AC-37 to that test's trace. | tests/it/terminal_map.rs:1466 |
| FND-003 | low | FR-017-AC-21's "requests whose wall-clock or memory ceilings differ start separate processes" is untested for the witness phase. No test holds members whose witness vectors are equal but whose ceilings differ, for example from two proof groups split by ceiling, and asserts they form separate witness launches. The coder states this gap. `shares_process` does check ceilings for both phases, so this is a coverage gap, not a known bug. | src/kani/run/execute/batch_tests/two_phase.rs:667-779 |

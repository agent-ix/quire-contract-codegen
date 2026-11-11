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

## Dispositions

Round 1, verified at head 7e0d82b3, rebased onto origin/main f38f8024.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The rows now mark the stand-in clauses Covered (IR-1022, #776) and record the installed-Kani controls FR-015-AC-41/119/126 as UNMET, with the TimedOut 13.2 GB and 13.4 GB measurement. They claim no fit in 16 GiB and say IR-1022 stays open. Two overclaims in FR-015-AC-9 and FR-017-AC-63 are recorded as FND-004 and FND-005 below. |
| FND-002 | fixed | `FR-029-AC-37` was added to the trace of `cause_matrix_preserves_literal_terminal_rows_and_pair_refusals`. |
| FND-003 | fixed | New test `held_members_with_different_ceilings_form_separate_witness_runs`. A mutant that ignores ceilings for the witness phase fails it. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-015-AC-9 now ends "Both vectors and both checks are implemented and covered (IR-1022, #776)". No test exercises either check on the witness vector: not the `HarnessSpec::new_constrained` recompute refusal (`InvalidExpression`), and not the V2 clause or collection `has_valid_provenance` refusal (`HarnessProvenanceMismatch`). The only witness-vector mutation in tests is `tc_035_the_frame_identity_names_none_of_the_harness_members`, which covers the StateFrame identity naming, not either check. Dropping `\|\| witness_options != identity.witness_options` from `new_constrained` would pass every test. Fix: add the two refusal tests, or keep the status text to what is tested. | spec/kani/functional/FR-015-bounded-kani-obligations.md:821 |
| FND-005 | low | FR-017-AC-63's status says clauses (a) to (f) are covered over stand-ins. Clause (a) still requires two launches "whose recorded ceilings, crate and target are equal". `KaniExecutionEvidence` records no crate or target directory, and the (a) test compares the ceilings, harness path and arguments only. Fix: drop "crate and target" from (a), or record and assert them. | spec/kani/functional/FR-017-kani-execution-evidence.md:915 |
| FND-006 | low | The new FR-017 Behavior sentence on `WitnessPhaseRefused` `code()` is unwrapped: line 268 is 141 characters, while the bullet around it wraps at 100. | spec/kani/functional/FR-017-kani-execution-evidence.md:268 |

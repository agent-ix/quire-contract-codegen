---
id: "SR-701"
title: "CG PR 210 gap analysis: FR-017 report criteria, TC-027 trace, IR-277 scope"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@363148d4e18e30c278b84c106a384503ed0ed47c; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/tests.md, src/kani_transcript.rs, src/kani_execution.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-027
    type: references
---

# SR-701: CG PR 210 gap analysis

## Summary

Ticket: IR-277 (also IR-288, IR-463). PR: agent-ix/quire-contract-codegen#210 at 363148d.
Planless gap analysis over the PR diff: each new or amended FR-017 criterion against its tagged
tests and the code, the matrix rows, and the tickets' stated scope. Plan completion: not assessed.

Checked and clean:

- FR-017-AC-18: `tc_027_a_report_that_changed_shape_is_refused_not_classified`,
  `tc_027_a_report_without_exactly_one_harness_is_refused`,
  `tc_027_an_unreadable_or_missing_report_is_refused_never_inconclusive` and
  `tc_027_execution_reads_only_the_report_its_own_run_exported` each assert a typed cause, and
  the code (`KaniHarnessReport::parse`, `classify_kani_run`) has a matching branch.
- FR-017-AC-19: `tc_027_the_launch_exports_the_report_after_the_harness_options`,
  `tc_027_the_report_is_read_bounded_and_refused_not_truncated` and the stale-report stand-in
  case discriminate (a stale verified report must yield `Missing`).
- FR-017-AC-20: the per-check view tests read real captures and pin the serialized keys.
- The new FR-017 SUCCESS-check count sentence is backed by `success_checks` assertions in
  `a_precondition_harness_with_no_checks_is_decided_by_its_cover_not_the_zero_checks_rule`,
  `a_report_with_no_successful_check_...` and the stand-in test; mutations of the rule are killed.
- The strip: FR-029, FR-030, TC-040 and TC-041 are byte-identical to main (empty diff); the
  matrix keeps them Planned; interface-001 has no terminal-map operation entries; `src/` and
  `tests/` contain no `terminal_value`, `proof_category` or `kani_terminal`. No requirement or
  AC id was removed or renumbered.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The TC-027 row of the test-case table lists no FR-017-AC-18, AC-19 or AC-20, while the FR-017 row of the same matrix marks those three Covered by TC-027. The two tables of one matrix disagree on what TC-027 verifies | spec/kani/matrix/tests.md:44, spec/kani/matrix/tests.md:23 |
| FND-002 | medium | TC-027's Implementation section names `a_zero_total_checks_summary_is_inconclusive_not_verified_even_with_every_cover_satisfied` for FR-017-AC-13. This PR renamed that test to `a_report_with_no_successful_check_is_inconclusive_not_verified_even_with_every_cover_satisfied`, so the trace points at a test that no longer exists | spec/kani/matrix/TC-027-kani-execution-evidence.md:112, src/kani_execution.rs:1080 |
| FND-003 | low | TC-027 Expected Results still describe cases this PR deleted or changed: "the three unreadable cover summaries" (:74), a bounded text "that still ends with the verdict" (:81), and captures that parse "to the expected typed transcript" with "missing cover summary" (:88-90). The procedure section was updated; these results were not | spec/kani/matrix/TC-027-kani-execution-evidence.md:70-90 |
| FND-004 | medium | IR-277 is only partly delivered, by ruling: its ticket criteria 1 (batching), 2 (no CG code outside one module reads console text; `kani_witness_join` still scans playback text), 4 (cap refusal; the capture still keeps the tail silently) and 5 are not met here, and AD-004 step 5 holds them. The branch name `feat/ir-277-...` links the PR to IR-277, so Linear's GitHub integration can move IR-277 to Done on merge. Keep IR-277 open (or split the delivered part) when this merges | src/kani_execution.rs:486-491, src/kani_witness_join.rs:187-347 |

## Verdict

Code coverage of the new criteria is real and discriminating. Fix the two matrix/trace defects
(FND-001, FND-002) and the stale Expected Results (FND-003) in the fix round; FND-004 is a
tracker action for the lead at merge.

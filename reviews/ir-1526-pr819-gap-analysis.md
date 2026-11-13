---
id: SR-17940486709564847047938
title: "IR-1526 PR #819 wide multiply probe and measured table gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #819 @ f261f105; IR-1526; FR-056-AC-5, FR-056-AC-10, FR-056-AC-11, FR-056-AC-12, TC-070; tests/fixtures/tractability/, tests/it/kani_tractability.rs, tests/it/kani_obligations_v2_clause_controls.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Planless gap analysis of PR #819, scoped to `git diff origin/main...HEAD`.
Plan completion: not assessed. The analysis covers FR-056-AC-5, AC-10, AC-11 and AC-12 and TC-070
steps 10 to 12 against the corpus, the new table fixture and the scoring and feature tests.

`quoin matrix --repo . --json` (quoin 0.30.1) lists every FR-056 criterion as `untagged` or
`method-without-symbol`, even though `tests/it/kani_tractability.rs` carries
`#[trace("FR-056-AC-*", "TC-070")]` on each test. That was already true on main and is outside this
diff, so it is not a finding here. The test-to-AC bindings below were checked by reading the code.

Focused run (shared with the code review): 19 passed, 1 ignored, among them
`tc_070_the_probe_corpus_rows_carry_the_decider_features` and
`tc_070_the_current_decider_scores_no_worse_than_the_baseline`.

## Verdict

**Mergeable from the gap side once the code-review findings are fixed.** This analysis adds one
medium finding of its own.

- AC-10 (Inspection). The three new rows record the decider's features, the measured `timed_out`
  ground truth, `wall_seconds` and the measurement source. Each was measured by a real run (the logs
  show `TimedOut` at about 600 s), and none takes its verdict from the decider. The status stays
  PARTIAL, which is honest: no Meter, decimal, pure-relational or non-integer multi-operation rows.
- AC-11. The scoring test now runs the table decider rather than the empty table. AlwaysRun regresses
  on the shipped corpus (false-accept 3), and so does AlwaysRefuse (false-refuse 10). The baseline of
  0/0 is the real score.
- AC-12. Unchanged and still exercised.
- AC-5. The 15-bit row against widths of 15, 23 and 31 follows the inclusive per-operation boundary.

Clean bindings: `tc_070_the_probe_corpus_rows_carry_the_decider_features` binds to AC-10 correctly,
because it proves the rows' features are the decider's. `tc_070_the_current_decider_scores_no_worse_than_the_baseline`
binds to AC-11 correctly.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | medium   | AC-10's open-items list in the Status leaves out the gap this PR exposes. The corpus now mixes general-route rows (3 timed_out) with native-route rows (10 verified or falsified), and the decider features carry no route. The ground truth of a row is therefore not a function of the features the decider is scored on. This is the same confound PR #818's review rejected for scalar rows, and it is what keeps the corpus from attributing the probe timeouts to mul (code-review FND-002). Add it to the AC-10 open items: either a route field in the corpus or decider features, or matching native-route measurements of the same clause shape. Leaving it out lets the 0/0 score read as validation of the mul row. | spec/kani/functional/FR-056-kani-tractability-refusal.md:134-144, tests/fixtures/tractability/corpus.json:372-469 |

## Dispositions

| FND | outcome | reason |
| --- | ------- | ------ |
| FND-001 | fixed | 28053a1b: the AC-10 Status open items now list "a route feature in the corpus features and in the decider, native-route wide cases", and say the general-route rows refuse native-route clauses with that false-refuse unmeasured. |

---
id: SR-17940486709564847047934
title: "IR-1526 PR #818 FR-056 corpus gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #818 @ c767df66; IR-1526; FR-056-AC-10, FR-056-AC-11, FR-056-AC-12, TC-070 step 10, tests/it/kani_tractability.rs corpus tests, tests/fixtures/tractability/"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Gap analysis of PR #818, scoped to `git diff origin/main...HEAD`. The PR changes
no production code, only the corpus and baseline fixtures and Status prose. I checked the criteria
the fixtures feed: FR-056-AC-10 (Inspection), AC-11 and AC-12 (Test), and TC-070 step 10.

## Verdict

**PASS** (no gap introduced by this PR). AC-10 is still bound by
`tc_070_every_corpus_row_names_its_measurement`, AC-11 by
`tc_070_the_current_decider_scores_no_worse_than_the_baseline` (prints 12 rows, 0/2 against
baseline 0/2), and AC-12 by `tc_070_a_new_corpus_row_and_a_second_decider_need_no_source_change`.
All 16 kani_tractability tests pass. The always-Run mutant still regresses only on the
synthetic corpus, a known residue that the PR #814 review deferred (FND-002 there), and the Status
says so. The inspection outcome of AC-10 for the two new rows (they are not V2 clauses) is
recorded once, in the code-review artifact SR-17940486709564847047933 FND-001, and is not
repeated here. No stubs and no reverse gaps.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low      | No findings (placeholder) | - |

## Coverage

- Criteria examined: FR-056-AC-10, FR-056-AC-11, FR-056-AC-12; context: TC-070 step 10.
- Production code changed: none.
- Plan completion: not assessed

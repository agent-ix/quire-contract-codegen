---
id: SR-8641
title: "IR-496 PR 367 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@0ce345ff1fb66f6bd9c5a445aeb6411cb9720a4b; spec/, src/, tests/"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

Planless repository matrix and reverse-gap audit, with focused source/test inspection for PR #367 at `0ce345ff1fb66f6bd9c5a445aeb6411cb9720a4b`. The computed matrix contains 323 untagged criteria; the changed FR-021 schedule slice includes untagged charge-boundary control flow and raised source-limit/deep-body criteria.

## Verdict

**FAIL** — the computed matrix has untagged criteria, including FR-021-AC-26 and AC-28, which fail the gap-analysis gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The computed matrix has 323 untagged criteria; the changed FR-021 slice leaves AC-26 and AC-28 without a bound test | FR-021-AC-26, FR-021-AC-28 |

## Coverage

- Reconciliation: `quire matrix --scope . --format json`, Quire 0.36.2; 738 criteria: 363 tagged, 323 untagged, 11 tagged-by-ignored-test, and 41 method-without-symbol. No Quoin run evidence was read. The untagged total includes criteria outside this PR; the review records the repository gate result and identifies AC-26/AC-28 in the changed feature area.
- Focused matrix bindings: FR-021-AC-25 and AC-27 have tagged tests in `src/oracle/function/schedule.rs`; FR-021-AC-34 has a public strict-reader test in `tests/it/exact_function_generation.rs`. FR-021-AC-26 and AC-28 are untagged. The new public schedule path also refuses literal leaves, normalizes equality identities, duplicates node/edge work charges across projection and schedule traversal, and sorts claims by name before declaration identity; these are code-review findings in SR-8640.
- Reverse-gap inventory: 122 public function declarations inventoried across `src/`; the new public `generate_function_schedules` API is owned by FR-021. Untraced behaviors: 0. Changed source stubs: 0. Changed test stubs: 0.
- Plan completion: not assessed
- Semantic review: skipped; not requested.

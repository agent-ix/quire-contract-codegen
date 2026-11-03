---
id: SR-2026110283828
title: "gap-analysis review of IR-1295 PR 725"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1295; tests/it/tc007_differential.rs; tests/it/bound_strategy_generation.rs; spec/core/matrix/TC-007-cross-backend-parity.md; spec/core/matrix/tests.md"
review_set: subset
---

## Summary

Scoped planless audit of FR-005-AC-3/4 and TC-007 against the computed matrix and the changed tests. AC-3 has one tagged test; AC-4 is deliberately OPEN and untagged. No new reverse ownership or stub gap was found in this diff.

## Verdict

PASS: no findings in the scoped method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Frozen head: `549a967d93aa0a680539a1acdfbd53b7f3b6a531`.
Plan completion: not assessed.
Computed Quoin matrix: FR-005-AC-3 tagged by `tc007_bound_census_and_executable_oracle_join_the_authoritative_register`; no bound run evidence. FR-005-AC-4 untagged and expressly OPEN. No new production code or CI workflow change in this diff.

## Reviewed units

- `FR-005-AC-3` (examined) at `spec/core/functional/FR-005-cli-conformance.md`: For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007 Differential Disposition Register holds exactly one row naming the case, the backends, and either a regression fixture that exists in the test tree or a documented profile/backend distinction; a differing case with no row, a row naming a fixture that does not exist, or a row with neither disposition fails the check.
- `FR-005-AC-4` (context_only) at `spec/core/functional/FR-005-cli-conformance.md`: OPEN: after IR-348 V1 retirement, agreement must be remeasured across surviving executable, proptest, Kani and coverage paths; removal alone is not agreement. Original criterion retained: Executable, proptest, Kani, and coverage semantics agree or retain an explicit difference.
- `TC-007` (examined) at `spec/core/matrix/TC-007-cross-backend-parity.md`: Compare every measured discrepancy with the authoritative register by the exact (case identifier, backend pair) key.

---
id: SR-2026110283830
title: "spec-review/evidence review of IR-1295 PR 725"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; IR-1295; tests/it/tc007_differential.rs; tests/it/bound_strategy_generation.rs; spec/core/matrix/TC-007-cross-backend-parity.md; spec/core/matrix/tests.md"
review_set: subset
---

## Summary

Reviewed whether the changed TC-007 and matrix evidence claims exceed the scoped execution. The claims correctly limit qualification to the bound-strategy census versus executable oracle and leave Kani, coverage, and AC-4 open.

## Verdict

PASS: no findings in the scoped method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Changed matrix status is Partial for FR-005-AC-3 and TC-007, Planned for FR-005-AC-4. No applicable AssuranceProfile found under the scoped `spec/` tree. Scoped Quire validation passed.

## Reviewed units

- `TC-007` (examined) at `spec/core/matrix/TC-007-cross-backend-parity.md`: Compare every measured discrepancy with the authoritative register by the exact (case identifier, backend pair) key.
- `FR-005-AC-3` (examined) at `spec/core/functional/FR-005-cli-conformance.md`: For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007 Differential Disposition Register holds exactly one row naming the case, the backends, and either a regression fixture that exists in the test tree or a documented profile/backend distinction; a differing case with no row, a row naming a fixture that does not exist, or a row with neither disposition fails the check.
- `FR-005-AC-4` (context_only) at `spec/core/functional/FR-005-cli-conformance.md`: OPEN: after IR-348 V1 retirement, agreement must be remeasured across surviving executable, proptest, Kani and coverage paths; removal alone is not agreement. Original criterion retained: Executable, proptest, Kani, and coverage semantics agree or retain an explicit difference.

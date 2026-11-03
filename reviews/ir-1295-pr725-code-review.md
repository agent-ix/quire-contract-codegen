---
id: SR-2026110283827
title: "code-review review of IR-1295 PR 725"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1295; tests/it/tc007_differential.rs; tests/it/bound_strategy_generation.rs; spec/core/matrix/TC-007-cross-backend-parity.md; spec/core/matrix/tests.md"
review_set: subset
---

## Summary

Reviewed the frozen PR #725 Rust test, helper exposure, fixture, register parser, generated scratch consumer, trace tags, and gate diff. The independent census and compiled oracle are meaningfully compared, but two fail-closed register defects remain.

## Verdict

CONDITIONAL: two medium defects require correction before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The unconditional measured.is_empty() assertion fails before the register join whenever a real difference is correctly assigned one valid fixture or distinction row. TC-007 must let the exact join determine whether a measured difference is dispositioned. | tests/it/tc007_differential.rs:437-444 |
| FND-002 | medium | The register parser stops at the first line without a leading pipe. An unexpected blank or malformed line can hide later rows, so a stale or duplicate row can evade the exact join. Reject unexpected content through the end of the register section. | tests/it/tc007_differential.rs:151-167 |

## Coverage

Frozen head: `549a967d93aa0a680539a1acdfbd53b7f3b6a531`.
The tag-versus-oracle comparison is independent; the typed runner verdict is a consistency observation. Seven in-domain identities and two outside-domain observations are asserted. `#![allow(dead_code)]` is confined to the generated scratch consumer. No Kani proof, retired V1 path, or gate narrowing appears. Focused TC-007 test was reported 1/1 by the coder; reviewer did not repeat it. `git diff --check` and scoped Quire validation passed.

## Reviewed units

- `FR-005-AC-3` (examined) at `spec/core/functional/FR-005-cli-conformance.md`: For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007 Differential Disposition Register holds exactly one row naming the case, the backends, and either a regression fixture that exists in the test tree or a documented profile/backend distinction; a differing case with no row, a row naming a fixture that does not exist, or a row with neither disposition fails the check.
- `TC-007` (examined) at `spec/core/matrix/TC-007-cross-backend-parity.md`: Compare every measured discrepancy with the authoritative register by the exact (case identifier, backend pair) key.
- `FR-005-AC-4` (context_only) at `spec/core/functional/FR-005-cli-conformance.md`: OPEN: after IR-348 V1 retirement, agreement must be remeasured across surviving executable, proptest, Kani and coverage paths; removal alone is not agreement. Original criterion retained: Executable, proptest, Kani, and coverage semantics agree or retain an explicit difference.

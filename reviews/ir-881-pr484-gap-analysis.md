---
id: SR-14311
title: "Gap analysis of IR-881 CG terminal mapping"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md, src/kani/terminal.rs, tests/it/terminal_map.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
---

## Summary

Ticket: IR-881. Reviewed the changed FR-030-AC-8 requirement, TC-041 procedure, tagged assertion, and production mapping at PR #484 head ab01e5eeeaa79ee310accd8bb986b393f41bf835. The changed behavior has an owning criterion and a substantive test.

## Verdict

**PASS for the PR diff** — No new trace, reverse-ownership, stub, or evidence gap was found in the changed mapping.

## Scope examined

- FR-030-AC-8 (examined): `Unavailable` with cause `kani_checker_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`.
- TC-041 step 6 and expected result 6 (examined): the three unavailable causes and results.
- `src/kani/terminal.rs:194-200` and `tests/it/terminal_map.rs:1207-1226` (examined): one production branch and exact, traced assertions.

## Coverage

Plan completion: not assessed.

One changed acceptance criterion, one changed production mapping, and one tagged test were examined. FR-030-AC-8 binds to `tc_041_an_absent_solver_or_backend_is_unsupported_with_its_cause`; the test calls the production map and asserts the checker, backend, and fallback results. No changed source behavior lacks an owning criterion, and neither changed function is a stub. The repository-wide strict matrix exits with 291 untagged criteria and 18 criteria tagged only by ignored tests; these existing repository findings are outside this PR diff and are not claimed clean here. Optional repository-wide semantic review was not selected.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

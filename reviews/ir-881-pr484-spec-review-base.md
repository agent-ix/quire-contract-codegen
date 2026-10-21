---
id: SR-14312
title: "Base specification review of IR-881 CG consumer rename"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md"
review_set: base
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
---

## Summary

Ticket: IR-881. Reviewed the FR-030 terminal table and AC-8, plus TC-041 step 6, against merged Contract IR's registered `kani_checker_absent` code and the unchanged terminal-result distinction.

## Verdict

**PASS** — The spec edits preserve the existing mapping contract while naming the newly registered IR code consistently. The two changed documents validate without grammar findings.

## Scope examined

- FR-030-AC-8 (examined): `Unavailable` with cause `kani_checker_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`.
- FR-030 behavior table unavailable rows (examined): checker absence maps to `SolverAbsent`; backend absence and other causes map to `BackendAbsent`.
- TC-041 step 6 and expected result 6 (examined): exercises and names the three unavailable outcomes.

## Evidence

The changed criterion has a concrete input and exact expected value for every branch. The edited TC procedure names all three inputs; its expected results and tagged Rust test cover them. `quire validate --scope .` on both changed spec files reported 2/2 grammar-clean documents and no findings. No new digest, pin, tracking ledger, or duplicated authority was introduced by the spec diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

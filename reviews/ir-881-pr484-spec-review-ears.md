---
id: SR-14313
title: "EARS review of IR-881 CG requirement edit"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-030-ir-outcome-terminal-map.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
---

## Summary

Ticket: IR-881. Examined the changed FR-030-AC-8 statement and its terminal mapping table for requirement grammar and semantic clarity.

## Verdict

**PASS** — The criterion gives an explicit condition and a determinate result for the checker code, backend code, and all other codes. The focused grammar validation reported zero findings.

## Scope examined

- FR-030-AC-8 (examined): `Unavailable` with cause `kani_checker_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`.
- FR-030 behavior table unavailable rows (examined): the same three cases and their proof-field absence.

## Evidence

`quire validate --scope .` on the changed FR and TC reported 2/2 grammar-clean documents and zero grammar findings. Semantic inspection found no unclear trigger, response, or contradictory branch in the edited statement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

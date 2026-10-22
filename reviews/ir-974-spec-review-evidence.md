---
id: SR-13298
title: "Evidence review of CG HP-2 SMT map"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@e5b93f6fa8f0ebc923d40fcb8f80992e3cd7ca82; spec/routed/functional/FR-045-hyper-smt-induction-map.md; spec/routed/matrix/TC-059-hyper-smt-induction-map.md; spec/routed/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-974. `quoin advise --json` recommends Test-class methods for both changed criteria and reports no authored-method mismatch or inconclusive recommendation. TC-059 plans solver-boundary byte and identity checks plus adverse response cases; the test and producer are correctly marked planned.

## Scope

- `FR-045-AC-1` (examined): "For an admitted HP-2 safety product and `k = 1`, CG requests QSL's base and step queries for the same product and depth, submits their canonical bytes unchanged to the selected solver, and maps `unsat`/`unsat` to `Inductive { depth: 1 }` with product and item identity retained. QSL FR-182-AC-7 then settles that outcome as `proved`, `decisive-witness`, `Uncertified`; CG does not construct the query or assign certification."
- `FR-045-AC-2` (examined): "With a base `sat` response, CG returns only a reconstructable product-path candidate for QSL replay; with base `unsat` and step `sat`, it returns `InductionNotClosed { depth: k }`; unknown, encoding refusal, resource stop, wrong-query association and missing response retain distinct typed outcomes and never produce `Inductive` or terminal `Refuted`. A resource stop maps to FR-182 V-7 incomplete, not inconclusive. The identities and `k` survive every branch."
- `TC-059` (examined): "Capture the bytes at the selected solver boundary and assert byte equality and exact product/item/depth association."
- `spec/routed/matrix/tests.md` (examined): "TC-059 | Map QSL-encoded HP-2 induction solver results | Integration | P0 | FR-045-AC-1, FR-045-AC-2 | 🚧 Planned (IR-974): QSL hyper-product query form and CG SMT producer absent"

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for evidence planning. The planned Integration test is a viable Test-class method for the criteria, contingent on the QSL query form and CG producer. This does not claim executed test evidence.

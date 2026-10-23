---
id: SR-13297
title: "EARS review of CG HP-2 SMT map"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@e5b93f6fa8f0ebc923d40fcb8f80992e3cd7ca82; spec/routed/functional/FR-045-hyper-smt-induction-map.md"
review_set: subset
---

## Summary

Ticket: IR-974. Examined FR-045 Description and Behavior with both criteria. Quire reports four changed documents grammar-clean with zero grammar findings; the conditional trigger and named CG response are concrete.

## Scope

- `FR-045` (examined): "When QSL supplies canonical SMT queries for an admitted HP-2 safety item's `HyperProduct`, CG's SMT backend SHALL submit those queries to its selected solver and return a typed method outcome to QSL's hyper settlement."
- `FR-045-AC-1` (examined): "For an admitted HP-2 safety product and `k = 1`, CG requests QSL's base and step queries for the same product and depth, submits their canonical bytes unchanged to the selected solver, and maps `unsat`/`unsat` to `Inductive { depth: 1 }` with product and item identity retained. QSL FR-182-AC-7 then settles that outcome as `proved`, `decisive-witness`, `Uncertified`; CG does not construct the query or assign certification."
- `FR-045-AC-2` (examined): "With a base `sat` response, CG returns only a reconstructable product-path candidate for QSL replay; with base `unsat` and step `sat`, it returns `InductionNotClosed { depth: k }`; unknown, encoding refusal, resource stop, wrong-query association and missing response retain distinct typed outcomes and never produce `Inductive` or terminal `Refuted`. A resource stop maps to FR-182 V-7 incomplete, not inconclusive. The identities and `k` survive every branch."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for EARS conformance. The description uses a conditional trigger and names CG's observable response; behavior obligations have named actor and checkable solver dispositions.

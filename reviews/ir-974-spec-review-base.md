---
id: SR-13296
title: "Base review of CG HP-2 SMT map"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@e5b93f6fa8f0ebc923d40fcb8f80992e3cd7ca82; spec/routed/functional/FR-045-hyper-smt-induction-map.md; spec/routed/matrix/TC-059-hyper-smt-induction-map.md; spec/routed/matrix/tests.md; spec/spec.md"
review_set: subset
---

## Summary

Ticket: IR-974. Examined FR-045, both criteria, TC-059 and both registry rows against QSL FR-315, FR-176, FR-127 and FR-182 at QSL origin/main 2478fe433. The IDs, planned matrix entries and CG/QSL ownership allocation are coherent. The test-result contradiction is recorded by the integrity lens in SR-13299.

## Scope

- `FR-045` (examined): "When QSL supplies canonical SMT queries for an admitted HP-2 safety item's `HyperProduct`, CG's SMT backend SHALL submit those queries to its selected solver and return a typed method outcome to QSL's hyper settlement."
- `FR-045-AC-1` (examined): "For an admitted HP-2 safety product and `k = 1`, CG requests QSL's base and step queries for the same product and depth, submits their canonical bytes unchanged to the selected solver, and maps `unsat`/`unsat` to `Inductive { depth: 1 }` with product and item identity retained. QSL FR-182-AC-7 then settles that outcome as `proved`, `decisive-witness`, `Uncertified`; CG does not construct the query or assign certification."
- `FR-045-AC-2` (examined): "With a base `sat` response, CG returns only a reconstructable product-path candidate for QSL replay; with base `unsat` and step `sat`, it returns `InductionNotClosed { depth: k }`; unknown, encoding refusal, resource stop, wrong-query association and missing response retain distinct typed outcomes and never produce `Inductive` or terminal `Refuted`. A resource stop maps to FR-182 V-7 incomplete, not inconclusive. The identities and `k` survive every branch."
- `TC-059` (examined): "Verify CG's solver submission and typed method outcome over QSL-owned canonical HP-2 product queries, without implementing a second encoder."
- `spec/routed/matrix/tests.md` (examined): "FR-045-AC-1, FR-045-AC-2 | TC-059 | 🚧 Planned (IR-974): waits for QSL-owned FR-315 HP-2 product query form; CG solver/map producer is not implemented"
- `spec/spec.md` (examined): "HP-2 SMT induction producer | FR-045 CG solver submission and outcome map over QSL-owned product queries | TC-059 planned"

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the base checklist. The changed criteria have a planned TC row, distinct input and outcome cases, and no copied QSL encoder or certificate authority. QSL FR-315 exposes encoding through `qsl-replay`; ADR-018 DS-2 requires CG to use that facade, and FR-060 T12-A enforces CG's replay adapter boundary. FR-045's FR-315 dependency preserves this already specified route; the missing facade spelling in FR-045 is not a new requirement gap. This is a specification review, not implementation acceptance. See SR-13299 for the integrity finding.

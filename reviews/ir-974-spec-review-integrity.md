---
id: SR-13299
title: "Integrity review of CG HP-2 SMT map"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@e5b93f6fa8f0ebc923d40fcb8f80992e3cd7ca82; spec/routed/functional/FR-045-hyper-smt-induction-map.md; spec/routed/matrix/TC-059-hyper-smt-induction-map.md; spec/routed/matrix/tests.md; spec/spec.md"
review_set: subset
---

## Summary

Ticket: IR-974. Checked ownership, outcome completeness and TC-059 internal consistency against QSL FR-315, FR-176, FR-127 and FR-182 at QSL origin/main 2478fe433. One TC expected-result sentence contradicts the specified V-7 resource-stop result and leaves encoding refusal outside its stated result set.

## Scope

- `FR-045` (examined): "When QSL supplies canonical SMT queries for an admitted HP-2 safety item's `HyperProduct`, CG's SMT backend SHALL submit those queries to its selected solver and return a typed method outcome to QSL's hyper settlement."
- `FR-045-AC-1` (examined): "For an admitted HP-2 safety product and `k = 1`, CG requests QSL's base and step queries for the same product and depth, submits their canonical bytes unchanged to the selected solver, and maps `unsat`/`unsat` to `Inductive { depth: 1 }` with product and item identity retained. QSL FR-182-AC-7 then settles that outcome as `proved`, `decisive-witness`, `Uncertified`; CG does not construct the query or assign certification."
- `FR-045-AC-2` (examined): "With a base `sat` response, CG returns only a reconstructable product-path candidate for QSL replay; with base `unsat` and step `sat`, it returns `InductionNotClosed { depth: k }`; unknown, encoding refusal, resource stop, wrong-query association and missing response retain distinct typed outcomes and never produce `Inductive` or terminal `Refuted`. A resource stop maps to FR-182 V-7 incomplete, not inconclusive. The identities and `k` survive every branch."
- `TC-059` (examined): "Only the two matching `unsat` queries produce `Inductive`. Every adverse branch retains its product, item and depth and remains eligible only for QSL-owned replay or inconclusive settlement."
- `spec/routed/matrix/tests.md` (examined): "FR-045-AC-1, FR-045-AC-2 | TC-059 | 🚧 Planned (IR-974): waits for QSL-owned FR-315 HP-2 product query form; CG solver/map producer is not implemented"
- `spec/spec.md` (examined): "HP-2 SMT induction producer | FR-045 CG solver submission and outcome map over QSL-owned product queries | TC-059 planned"

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-059 Expected Results limits every adverse branch to replay or inconclusive settlement, contradicting its resource-stop V-7 incomplete assertion and omitting the encoding-refusal outcome. State the distinct expected destinations explicitly. | spec/routed/matrix/TC-059-hyper-smt-induction-map.md:34-38; FR-045-AC-2; QSL FR-182 V-7 | wrong-requirement |

## Verdict

FAIL for integrity until TC-059's expected results include V-7 incomplete and encoding refusal as distinct outcomes. FR-045's outcome table and AC-2 preserve this distinction; the inconsistency is confined to the TC summary.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At PR #526 head `49a45c9cd983be42a11590625fc1f2d9b7b2efbb`, TC-059 Expected Results separately sends a base `sat` candidate to QSL replay, step `sat`/unknown/unusable results to inconclusive, resource stops to FR-182 V-7 incomplete, and encoding refusal to its original typed refusal. |

## Current verdict

PASS after disposition round 1. The original finding above remains as the review record; the changed TC-059 sentence resolves its contradiction without moving QSL encoding or settlement ownership to CG.

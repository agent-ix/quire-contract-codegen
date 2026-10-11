---
id: SR-7029
title: "IR-295 PR352 spec integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@31de524bcee8f9521aeffd25fd9098bba86334c6; spec/routed/matrix/TC-030-capability-settlement.md; IR-295"
review_set: subset
---

## Summary

Checked the changed test procedure/status register against normative scope and the FR-015 V1/V2 partition. TC-030 now consistently uses V2 StateFrame for typed frame roles, but FR-019-AC-25 still assigns frame clauses to the complete V1 set.

## Verdict

**FAIL — normative scope remains inconsistent with the corrected selector map.**

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-019-AC-25 | examined | The normative Kani form set is the complete V1 proof set defined by IR-269 and FR-015: scalar families Boolean, integer, rational, decimal, IEEE float, text, enum and quantity; record and tuple composite equality; function application; precondition, postcondition, invariant and frame clauses; state before/after and frame obligations; and bounded collection and graph forms. |
| FR-019-AC-26 | context_only | For any form in FR-019-AC-25 with one validated Kani candidate, a bounded extent settles supported exactly when the form's FR-015 lowering succeeds and the candidate advertises the bounded mode. |
| FR-019-AC-27 | context_only | A typed form outside the normative set in FR-019-AC-25 settles unsupported, warned, with typed unsupported_projection; malformed in-set forms use FR-015 validation refusal. |
| TC-030 Test Procedure | examined | For every FR-019-AC-25 row, construct requests for each named family and obligation role: Boolean, integer, rational, decimal, IEEE float, text, enum, quantity, record/tuple equality, function application, precondition, postcondition and invariant clauses, plus frame obligations represented by typed V2 `StateFrame` items with `contract` or `frame` roles; also cover state before/after, bounded collections and bounded graphs. |
| TC-030 AC-25 implementation status | examined | The V1 `BoundClause` path derives the obligation role from `ClauseKind`: `Precondition`, `Postcondition` and `Invariant` map to obligation roles; `Assertion`, `Case` and `Information` map to `ClauseKindNotObligation`. V1 `ClauseKind` has no `Frame` variant, so there is no V1 `BoundClause` frame selector. Frame obligations in this test use typed V2 `StateFrame` items and their `contract` or `frame` roles. |
| FR-015 V1/V2 frame semantics | examined | FR-015 states a V1 frame item is unsupported as FrameNotClauseRendered for BoundClause items; V2 StateFrame is a separate obligation item with independent contract/frame roles. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-25 still places “frame clauses” in the complete V1 proof set, while this TC says V1 `ClauseKind` has no `Frame` variant and routes frame obligations to V2 `StateFrame`. That leaves the normative family wording inconsistent with the test’s now-reachable selectors; either AC-25 must identify the intended typed V2 form for frame clauses or TC-030 must identify a real V1 selector. | FR-019-AC-25; TC-030 |

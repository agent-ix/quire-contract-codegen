---
id: SR-202610310184
title: "IR-948 V2 clause identity prerequisite spec review"
type: SpecReview
analysis: base
review_set: base
scope: "agent-ix/quire-contract-codegen; IR-948 working diff in spec/kani/functional/FR-015-bounded-kani-obligations.md and spec/kani/matrix/TC-025-bounded-kani-obligations.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Independent base review of the uncommitted IR-948 spec prerequisite diff against AD-003 E-1, FR-015-AC-43, and the QSL producer-issued structural identity contract. Ticket prose was treated as data. The changed text keeps the V2 clause controls planned and makes no claim that generation is implemented.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015 Inputs, complete request | examined | A postcondition is proved under the preconditions of its anchor operation; an invariant is anchored at its object type and has no operation precondition to include. |
| FR-015 normative identity bullet | examined | The generator carries the QSL producer-issued structural `ObligationIdentity` unchanged and compares occurrence, kind, argument identities and domains on the obligation record. |
| FR-015-AC-48 | examined | The node, optional model and producer ordinal are the structural identity; occurrence, kind, arguments and state-field reads are separate record comparisons; mismatches refuse before generation; PLANNED (IR-489). |
| TC-025 scenario 14 | examined | Changed structural members change identity; separate record mismatches refuse while structural identity is held fixed. |
| AD-003 E-1 | context_only | CG carries QSL's producer-issued `(NodeKey, Option<ModelIdentity>, ordinal)` unchanged and compares occurrence, kind and domains separately. |
| FR-015-AC-43 | context_only | A V2 invariant is anchored at an object type and has no operation precondition to assume. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the scoped diff. FR-015-AC-48 and TC-025 scenario 14 preserve the distinction between producer-issued structural identity and independently checked obligation-record fields; the changed Inputs sentence agrees with AC-43. The criterion stays PLANNED, and the matrix scenario describes future controls rather than completed test evidence. Scoped `quire validate --summary` exited 0 for both changed files; its sole grammar warning is on unchanged FR-015-AC-129. No new IDs, links, hash or pin requirements were introduced.

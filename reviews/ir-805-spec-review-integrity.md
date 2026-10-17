---
id: "SR-12102"
title: "IR-805 spec-review/integrity review of reference-graph reachability Kani claims"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@edbe6aabdd756a864482c2b73b1c0c113b101894; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-12102: IR-805 spec-review/integrity review

## Summary

Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.
Method: spec-review/integrity (completeness, internal consistency, atomicity).

## Scope

- FR-015 Inputs :204-210 (examined), FR-015 description :136-148 (examined), FR-015-AC-98 (examined).
- FR-015-AC-96, FR-015-AC-97, and the collection clauses of FR-015-AC-99 to FR-015-AC-102 (examined): unchanged from origin/main; IR-268 collection half (CG #393/#396) remains consistent.
- tests.md :94, :95 (examined): consistent.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Inputs bullet says a graph item carries "the finite `model_population` binding and `Type` occurrence of each of its start and target types", i.e. a caller-named FR-053 selector, while :140-144 says IR has no accessor mapping a reaches claim to its population binding and "an item lacking either settles `unsupported`". It is undefined whether a caller-named selector suffices (-> Finite, harness) or the missing mapping always refuses. If the former, a caller can name any `Population<Order>[N]` unrelated to the claim, which is a caller-chosen bound like the forbidden `max_expansions`; AC-98 lists no refusal for a selection not bound to the claim. | spec/kani/functional/FR-015-bounded-kani-obligations.md:204, spec/kani/functional/FR-015-bounded-kani-obligations.md:140, spec/kani/functional/FR-015-bounded-kani-obligations.md:766 |

## Verdict

Revise before accept: settle which input authority binds a claim to its population.

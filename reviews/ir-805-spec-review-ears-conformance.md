---
id: "SR-12104"
title: "IR-805 spec-review/ears-conformance review of reference-graph reachability Kani claims"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@edbe6aabdd756a864482c2b73b1c0c113b101894; spec/kani/functional/FR-015-bounded-kani-obligations.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

# SR-12104: IR-805 spec-review/ears-conformance review

## Summary

Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.
Method: spec-review/ears-conformance over the new and rewritten criteria.

## Scope

- FR-015-AC-98 (examined), FR-015-AC-105 (examined), FR-015-AC-106 (examined), FR-015-AC-107 (examined).
- Graph clauses of FR-015-AC-100 and FR-015-AC-101 (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-98 is compound (harness precondition, inclusive bounds, positive-length semantics, `requires_bound` triggers, `unsupported` triggers) and makes "an absent typed relationship-end multiplicity accessor" a normative trigger, so the clause names a missing upstream API rather than an observable input condition and goes vacuous once IR adds it. Split the refusal triggers and phrase the trigger as "relationship-end multiplicity not available from typed Contract IR". | spec/kani/functional/FR-015-bounded-kani-obligations.md:766 |

## Verdict

Accept with the low finding noted; the other examined criteria are single-response and testable.

## Dispositions

Reviewed at e1873b156ea042a0b76fedf2a804becf17ac89e0 (round 1; fix commits d088e9c, e1873b1).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d088e9c..e1873b1: AC-98's refusal triggers moved to AC-108; the end trigger is now an input condition (multiplicity IR does not expose as a typed value, IR-851). |

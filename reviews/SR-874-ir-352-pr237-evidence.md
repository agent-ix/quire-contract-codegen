---
id: "SR-874"
title: "CG PR 237 spec review (evidence): verification methods for FR-021-AC-19 to AC-21"
type: SpecReview
analysis: evidence
review_set: subset
scope: "agent-ix/quire-contract-codegen@6d0e02a4770c6b77bc4b08ea46a3978015c6c1bc; FR-021-AC-19, FR-021-AC-20, FR-021-AC-21 (method column) and TC-031 step 8; quoin advise --json (quoin 0.24.1) over the head"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

# SR-874: CG PR 237 spec review, evidence

## Summary

Ticket: IR-352. `quoin advise --json` at head reports no mismatch for FR-021-AC-19, AC-20 or AC-21:
each is authored `Test` and each recommendation set includes a `Test`-class method (unit-testing or
e2e-testing). The evidence each needs is an automated test over emitted text or generator source,
the same kind the planner accepted for #232 because a foreign `#[non_exhaustive]` variant cannot be
built from a test crate. TC-031 step 8 states that limit instead of implying the unknown variant is
exercised. The pre-existing FR-021-AC-16 is authored `Inspection` for a similar text check and is an
advise mismatch on main already; not introduced here.

## Verdict

Clean. The methods fit and the stated evidence limit is honest.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Dispositions

Round 1, reviewed at cac5002cc137ad6297def98b225b9394b0673fef: no findings to dispose. quoin advise is unaffected by the rewrite (methods unchanged, still Test).

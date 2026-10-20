---
id: SR-13459
title: "CG PR 467 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; src/oracle/scalar/mod.rs, src/kani/, src/routed/, tests/exact_scalar_support/, tests/it/"
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-014
  type: reviews
---

## Summary

Ticket: IR-914. PR #467 at 7532795d. The computed Quire matrix binds FR-014-AC-35 to three tests, AC-36 to one, and AC-37 to three. The production paths have matching FR-014 ownership. The two gaps below are in test evidence; no plan was supplied. Plan completion: not assessed.

## Verdict

CONDITIONAL

## Examined

- FR-014-AC-35 (examined): Boolean `and`, `or`, `not`, `implies`, `eq` and `ne` nodes and integer `eq`, `ne`, `lt`, `le`, `gt` and `ge` nodes each derive a descriptor and generate an `ir_confirmed` oracle; each connective's oracle calls `evaluate_boolean` or `evaluate_boolean_short_circuit`, and each `eq`/`ne` oracle calls `check_equality` and `CheckedEquality::evaluate`, each returning `Outcome<bool>`.
- FR-014-AC-36 (examined): Every oracle's source map declares the node's implication-consequent count, one evaluation-entry probe on the function-entry line disjoint from every consequent region, and one entry-token probe inside the exact region of each consequent; a dropped or duplicated region does not change the declared count.
- FR-014-AC-37 (examined): A differential corpus covering every node FR-014-AC-35 names compiles against the runtime alone, and each oracle's outcome, admitted charges and consumed counters equal direct runtime execution. This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 (generated-oracle against QSL agreement), and FR-014-AC-47 keeps CG's artifacts from claiming it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The corpus exercises stopping right operands only for implies. It has no admitted lazy and/or nodes or generated-vs-runtime cases for the short-circuit And and Or arms, so these newly added branches can regress while the tagged Boolean tests pass. | tests/exact_scalar_support/boolean_agreement_cases.rs:27 |
| FND-002 | medium | The source-map test checks probe lines and roles but never checks startColumn/endColumn against the executable token, never checks every oracle entry, and never removes or duplicates a region to assert the declared consequent count stays fixed. The tagged test therefore does not establish the exact-token and independent-census parts of AC-36. | tests/it/exact_scalar_generation.rs:3270 |

## Coverage

Computed matrix: AC-35 3 binders; AC-36 1 binder; AC-37 3 binders. Plan completion: not assessed.

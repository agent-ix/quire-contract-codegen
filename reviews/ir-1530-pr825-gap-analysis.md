---
id: SR-17940486709564847047962
title: "IR-1530 PR #825 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #825 at 212a7dfd, git diff origin/main...HEAD; IR-1530; FR-056-AC-15; tests/it/kani_tractability.rs, docs/qualification/ir1530-general-route-scalar-table.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
---

## Summary

Ticket: IR-1530. Plan completion: not assessed. The changed data, the shipped scalar table embedded by
`TractabilityTable::shipped_general_route_scalar`, is owned by FR-056-AC-15. Its three binders are
`tc_070_the_embedded_general_route_table_is_the_docs_file`,
`tc_070_a_general_route_scalar_harness_is_refused_before_kani_with_its_row` and
`tc_070_native_route_and_v2_clause_features_are_not_refused_by_the_general_row`. All three were
updated to the two-row table with substantive assertions: row count 2; the refused harness reports
the add row (`operation_ids == [ADD]`, `max_operand_bits == 2`); add refused at 2, 3 and 4 bits and
run at 1; sub run at 3 and refused at 4. No production code changed, there are no stubs and no
tautologies.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. Coverage of the two-row table is complete at the decider and wrapper levels.

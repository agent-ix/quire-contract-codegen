---
id: SR-2026110283871
title: "IR-1346 PR 747 integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; IR-1346; PR 747; spec/oracle/functional/FR-018-composite-equality-oracles.md; spec/oracle/matrix/TC-029-composite-equality-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-029
    type: references
---

## Summary

Reviewed the two changed specification files at PR 747 head d54732b3309f60deaef7a2e20a0fead67404f8b6. No applicable AssuranceProfile was declared under spec/.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS. The new direct-literal branch and existing reference/conversion branch are disjoint and keep their distinct sources of type authority. The negative descriptor mutation can fail at OperandTypeMismatch before type resolution, so the expected refusal is coherent.

## Coverage

Examined FR-018 Behavior operand paragraph, FR-018-AC-47, FR-018-AC-15 as context, and TC-029 inline Integer literal operand controls. Scoped Quire validation and diff whitespace check passed. No Cargo test, Kani or aggregate gate was run for this spec-only review.

### Examined units

- FR-018-AC-47 (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: For a genuine QSL source whose checked `binary` equality has inline Integer literal operands, CG reads each admitted IR `CheckedTerm::Literal.type_id` and generates the item when both ids equal the descriptor's source types. An independently supplied Integer value pair evaluates through the generated oracle to the same completed Boolean value as direct runtime equality. Changing one descriptor source type to a different admitted type while retaining the original package refuses that item as `OperandTypeMismatch` at the changed operand position, with `found` equal to the original literal `type_id`, no generated symbol, and an unchanged healthy sibling. A literal's kind, value, outer result type, or descriptor alone cannot supply its type.
- TC-029 (examined), spec/oracle/matrix/TC-029-composite-equality-oracles.md: For FR-018-AC-47, compile a genuine QSL source containing a closed Integer equality over two inline literals. Read its unchanged checked package through Contract IR and select the emitted `binary` expression and the authentic Integer type node. Confirm the admitted `CheckedTerm::Literal` operands retain that type node id. Give CG a descriptor naming it on both sides and require a generated claim and oracle. Execute the generated oracle on an independently supplied Integer pair and compare its completed Boolean result with direct runtime equality for the same pair and operator; the generated so
- FR-018-AC-15 (context only): the reference/conversion path and exact mismatch behavior remain in force.

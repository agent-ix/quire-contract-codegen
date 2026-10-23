---
id: SR-16527
title: "IR-973 CG542 incremental acceptance and gap review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR 542 head 1af1e8a7cb6b48293a3d9bb638434cb1b7a0475e; incremental changed-code scope, not a repository-wide assurance audit"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-018", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

The changed scalar/equality branches have typed term dispatch and focused tests. The new structural integration test covers all three named consumers, but the untouched function consumer still uses raw JSON term keys and a local graph alias at this draft head, so the added test is red and the ticket's cross-consumer acceptance is incomplete.

## Verdict

FAIL — the draft cannot satisfy its own structural test or the three-consumer IR-973 acceptance until the function consumer is ported. This finding is expected to be dispositioned on the author's next head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new three-consumer structural test necessarily fails: the unchanged function module still performs raw JSON `term` dispatch and declares a separate node-map alias. Complete the typed function port and rerun its semantic output/refusal controls before claiming IR-973 acceptance. | tests/it/checked_term_consumers.rs:157; src/oracle/function/mod.rs:898 |

## Coverage

Incremental scope: compared the IR-973 acceptance text with the thirteen changed files, checked FR-014 bound/narrowing and FR-018 conversion operand behavior against typed consumers, and inspected the untouched function module because the new structural test reads it. The original scalar and equality tests retain trace tags; new association tests have suite-only TC traces and explicitly do not claim generated behavior. The admitted opaque metadata control re-reads a public owner package and checks both no-extra-consumer and real-argument-consumer outcomes. Damaged private graph fixtures are not counted as admission evidence. No test matrix was authored. Optional broad semantic audit was not requested. The author's reported focused results were not rerun or credited as a final gate.

Plan completion: not assessed

## Examined scope and bindings

```yaml
scope:
- id: FR-014-AC-27
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A node whose result is typed by a scalar type and that only narrowing conversions (`quire.op.numeric.narrow`, exactly one argument referencing the node) consume takes the bound they narrow it to as its result bound: `x + y` over `Int[0, 9]` and `Int[10, 20]` narrowed to `Int[10, 29]` generates with checked bounds `[10, 29]`, `[0, 9]`, `[10, 20]` and derives the descriptor over `[10, 29]`, a descriptor over `[0, 9]` is `BoundMismatch` against `[10, 29]`, and `-z` over `Int[0, 9]` narrowed to `Int[-9, 0]` checks `[-9, 0]` then `[0, 9]`.'
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference to a `convert` expression node is read as the type of what it converts, following nested conversions to the first node that is not a conversion, and a reference to any other node, including a non-`convert` application, as that node''s own `semantic_type`. A descriptor whose `source_type` is the read type generates, and any other source is refused as an operand-type disagreement at that position, reporting the read type as found.'
- id: FR-021-AC-12
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A declared function whose own body fails to lower — an unlowerable node, a form none of this generator's classifiers admits, or a nested `call` naming a function absent from the request — refuses every item bound to that function with a typed reason, without changing an unrelated package's items.
```

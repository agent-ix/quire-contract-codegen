---
id: SR-16526
title: "IR-973 CG542 independent second code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR 542 head 1af1e8a7cb6b48293a3d9bb638434cb1b7a0475e; base 1187f6cc52dd6a63c3655b10f1d83d863888e283; changed Kani generation, checked graph, scalar and equality oracle source and tests"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-018", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

Second independent diff-scoped review of the frozen draft head. The changed scalar, equality, and narrow Kani paths use owner `CheckedTerm` variants and a borrowed `CheckedGraph`. The admitted opaque-operation control separates metadata from actual argument references. One source-level explanation now contradicts the implementation.

## Verdict

CONDITIONAL — one low documentation defect. This is a draft review, not a final-head gate or acceptance of the as-yet-unported function consumer.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Scalar module documentation still says the shape classifiers dispatch on raw `term`/`operator`/`arguments`, although the changed implementation now dispatches through `CheckedTerm::Application`; update the description to match the typed path. | src/oracle/scalar/mod.rs:24 |

## Coverage

Examined every changed Rust path in the thirteen-file diff: Kani frame/negotiate/scalar, the new graph view and its tests, scalar and equality oracle source and tests, and the structural integration test. Checked owner-retained node/term association, key order, foreign-key refusal, typed operand and bound reads, no local term decoder, explicit damaged fixture separation, and the admitted opaque metadata discriminator. No vendored file, new dependency, CI-workflow narrowing, unsafe block, or compatibility layer appears in the diff. `git diff --check` passed. I did not run a full native, Kani, or spec gate; the author owns final-head qualification after the branch is completed and rebased. No applicable AssuranceProfile with review selection was found under this repository's spec tree.

Plan completion: not assessed

## Examined scope and bindings

```yaml
scope:
- id: FR-014-AC-16
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A bound is read from `binding` members looked up by name in any order (`min`/`max`, `numerator_min` through `denominator_max`, `coefficient_min` through `rounding`, `rounding`, `min`/`max`/`text_profile`); a bare literal member, a missing, duplicate or unlisted name is refused as an unreadable bound.
- id: FR-014-AC-27
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A node whose result is typed by a scalar type and that only narrowing conversions (`quire.op.numeric.narrow`, exactly one argument referencing the node) consume takes the bound they narrow it to as its result bound: `x + y` over `Int[0, 9]` and `Int[10, 20]` narrowed to `Int[10, 29]` generates with checked bounds `[10, 29]`, `[0, 9]`, `[10, 20]` and derives the descriptor over `[10, 29]`, a descriptor over `[0, 9]` is `BoundMismatch` against `[10, 29]`, and `-z` over `Int[0, 9]` narrowed to `Int[-9, 0]` checks `[-9, 0]` then `[0, 9]`.'
- id: FR-014-AC-18
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`derive_exact_scalar_items` returns, per node id in input order, the descriptor determined by the node''s `operation.identity`, `operation.mode`, `operation.laws`, operand forms and the one bound of the needed form on its result type; a node that is not an application, has no identity, names an identity outside the derivable set, has operand forms, a law or a mode that select no parameter, is refused with the matching `ClaimDerivationRefusal` and no descriptor; a node that is absent, does not lower or has a refused bound (missing, repeated or unreadable) is refused with the `ExactScal'
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference to a `convert` expression node is read as the type of what it converts, following nested conversions to the first node that is not a conversion, and a reference to any other node, including a non-`convert` application, as that node''s own `semantic_type`. A descriptor whose `source_type` is the read type generates, and any other source is refused as an operand-type disagreement at that position, reporting the read type as found.'
- id: FR-021-AC-12
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: context_only
  excerpt: A declared function whose own body fails to lower — an unlowerable node, a form none of this generator's classifiers admits, or a nested `call` naming a function absent from the request — refuses every item bound to that function with a typed reason, without changing an unrelated package's items.
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At `b57743ddf4cdb0368ef09b6f4fffd0526443627b`, `src/oracle/scalar/mod.rs:24-28` now states that shape classifiers match retained `CheckedTerm` and closed `ApplicationOperator`, borrowing typed arguments through `CheckedGraph`. |

## Narrow carry — round 1

The eight original source/test commits are `=` under `git range-diff 1187f6cc..1af1e8a7 a3aad3b7..b57743dd`. The new head is based directly on actual merged #546 `a3aad3b757bdd3b9028094c0b562c26970aaa3d3`; no rebase conflict rewrite to those eight commits was observed. I inspected the new function port and extra test adaptations. No new code/Rust finding in the changed function path. Typed `source_call_matches` retains exact arity, canonical represented-key lookup and ordered arguments; `source_function_node_id` still walks node-key order and rejects a second same-name/body match; first binding and dangling/foreign-key controls exercise the changed typed helpers. The checked source and lowered package remain separately owned. The integer literal lexer retains raw accepted spellings while refusing invalid spelling and unsupported kinds. `git diff --check` passes. Author's focused logs show fmt and workspace lib/tests Clippy PASS, 38 function controls, and before/after generated source, claim map and location map byte equality; these are read as data, not a final aggregate gate. Original findings above remain unchanged.

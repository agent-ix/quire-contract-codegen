---
id: SR-13470
title: 'spec-review-scope-boundary review of IR-914 PR #469'
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md,
  spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md,
  spec/oracle/matrix/tests.md
review_set: subset
---

## Summary

Reviewed PR #469 exact head ccf78124a61044fc247d886161684ca882fc1995 against the published QSL FR-341 state-clause shape and Contract IR typed-reader boundary. Ticket: IR-914.

## Examined

- FR-014-AC-38 (examined): PLANNED (IR-489). The Boolean condition in the third argument of a `state`/`state_clause` node's `quire.op.state.clause` body, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle over its connectives, integer comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`. When the condition is a reference to an admitted Boolean expression node, the oracle uses that node's expression, not the reference as a Boolean variable; an inline condition uses its own expression. In both cases the claim and generated oracle retain the clause 
- FR-015-AC-40 (examined): A harness for a V2 clause whose condition, inline or referenced from an admitted Boolean expression node, contains Boolean connectives and bounded-integer comparisons (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds an FR-014 oracle of that condition byte-identical to the oracle crate's clause-identified function, as FR-015-AC-20 states for a scalar claim; the clause-identified oracle needs FR-014-AC-38. PLANNED (IR-489).
- TC-024-step-3 (examined): 3. From a QSL-emitted, strict-reader-admitted V2 state-clause package, select a clause whose `quire.op.state.clause` body's third argument references a separate Boolean expression node. Request the clause node id and assert that its descriptor and generated `ir_confirmed` `Outcome<bool>` oracle evaluate the referenced expression, while the claim and oracle retain the clause node id rather than the expression node id. Exercise current and `pre(...)` field reads at both declared range endpoints. In a separate admitted inline-condition fixture, assert the same clause identity rule. A request for 
- FR-014-AC-38-matrix (examined): | FR-014 | FR-014-AC-38 | TC-024 | 🚧 Planned; the oracle of an inline V2 clause term (IR-489, FR-015-AC-40) |
- FR-014-description (examined): - Planned (IR-489, IR-815): where FR-015 names a V2 clause claim, or FR-002 (IR-815) selects a V2 precondition or postcondition for its harness, the generator shall derive a descriptor and generate an oracle for the Boolean condition in the third argument of that `state`/`state_clause` node's `quire.op.state.clause` body. When that argument is a reference to an admitted Boolean expression node, the referenced node supplies the condition expression; an inline condition supplies its own expression. The oracle claim is addressed by the clause node id in either case, without admitting the `state` 
- FR-015-input (examined): assumed. The clause body's third argument is its Boolean condition: it may be an inline term or a reference to an admitted Boolean expression node, whose expression supplies the condition without replacing the clause node as the claim identity. The condition may hold Boolean connectives, bounded-integer comparisons and the integer arithmetic expressions add, subtract, multiply and negate.

## Verdict

**PASS** — no findings in this method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Boundary evidence

QSL FR-341 defines the clause body’s third argument as its Boolean condition and the clause node as the claim. The changed wording keeps the clause id for both condition forms and leaves state-node independent scalar claims refused.

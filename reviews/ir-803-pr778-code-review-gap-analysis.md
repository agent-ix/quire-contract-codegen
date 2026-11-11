---
id: SR-11667
title: "IR-803 PR #778 typed composite operator: gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #778 @ a70f29af vs origin/main; IR-803; src/replay/composite_builder.rs, tests/it/composite_original_eq.rs; criteria FR-033-AC-1, FR-014-AC-2, TC-048, TC-024; spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

Ticket: IR-803. This is a planless, diff-scoped gap analysis of PR #778 against the criteria its new test claims (FR-014-AC-2, FR-033-AC-1, TC-024, TC-048) and against the ticket's acceptance: a typed operator on the public accessor result (delivered in IR #410), tests for Eq and Ne, and no raw JSON re-read left in CG composite_builder.rs. Reconciliation used `quire matrix --scope .` on the PR head (quoin 0.29.1), which still lists tagged tests for both FR-033-AC-1 and FR-014-AC-2, plus direct reading of the tests, the code and the IR accessor at quire-contract-model c8dca8c6. Plan completion was not assessed. No spec file changes in this PR, so spec-review was not run.

## Verdict

CONDITIONAL. The code meets the ticket acceptance:

- composite_builder.rs reads no identity.
- Eq and Ne are each exercised by tagged tests: `original_operator_mismatch_refuses_with_typed_operators`, the Ne parameter/literal tests and the Eq constructor tests.
- The Eq/Ne swap mutant fails 20 of 25 focused tests.

FR-033-AC-1's text ("structural.eq is Equal and structural.ne is NotEqual", "OperatorMismatch carrying the requested and actual operators") and its PARTIAL status are still accurate. TC-048 needs no change.

The spec text for FR-014-AC-2 is now wrong about this reader, and the new test is tagged with that AC while asserting the opposite refusal (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Two spec passages are stale for the replay composite-equality builder: FR-014-AC-2 (FR-014-exact-scalar-oracles.md:644) and the FR-014-AC-2 matrix row (spec/oracle/matrix/tests.md:18). Both still list the builder among the closed-whitelist readers that "read a closed set of operation identities". FR-014-AC-2 also says it refuses a conversion as `UnsupportedNodeOperation`. After this PR the builder reads no identity. A conversion, like every other catalogued non-eq/ne application, refuses as `CompositeBuildError::Operands(IneligibleOperator)` from IR's projection. Because the projection already ran first, that was also the observable result on main. The new test `a_non_equality_operation_refuses_as_an_ineligible_operator` is tagged FR-014-AC-2 and TC-024 but asserts `Operands(IneligibleOperator)`, which contradicts the AC text it claims. Fix (not done in this review): amend the FR-014-AC-2 sentence and the tests.md:18 row so this builder refuses through the checked projection as `Operands(IneligibleOperator)`, either in this PR or under a tracked follow-up. Otherwise drop FR-014-AC-2/TC-024 from the new test's trace. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:644 |
| FND-002 | low | FR-014-AC-2 names the conversion case for this reader, but the composite builder no longer has a conversion test. The deleted unit test used `quire.op.quantity.convert`, and the new test uses `quire.op.integer.lt`. Both go through the same IR `from_identity` refusal arm, so the risk is small. Fix: add a `quantity.convert` application to the new test (or a sibling) if FR-014-AC-2 keeps naming the conversion disposition of this builder. | tests/it/composite_original_eq.rs:602-628 |

## Dispositions

Round 1. This is a read-only check at head 030f7ffd92e603f037f18e0204fb046edc1ba861: fix commit 030f7ffd on a70f29af, with origin/main d7e0505f. No cargo was run, so the reviewer did not execute the new conversion test.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The FR-014-AC-2 sentence now names only two identity readers that refuse a conversion: `StateComparison::from_operation` returns `None` and `resolve_predicate` returns `UnsupportedShape`. Both claims hold in the source. `from_operation` (src/kani/generate/frame.rs:121) goes through `operator_catalog::Operator::from_identity`, whose ten entries include no quantity identity. `resolve_predicate` (src/kani/generate/collection.rs:943) falls to its `_ => UnsupportedShape` arm. The sentence also says the replay composite-equality builder reads no identity and that a conversion is refused by the checked projection as `Operands(IneligibleOperator)` before any request. That matches `selected_claim` (the projection is the first step, and `CheckedCompositeOperator::from_identity` covers eq/ne only). The tests.md:18 row matches, except for the test name (FND-003). |
| FND-002 | fixed | The new `a_quantity_conversion_refuses_as_an_ineligible_operator` compiles a `convert<Quantity<m>>(a)` function. Its source is byte-identical to the one in tests/it/scalar_source_forms.rs:204. `application_with_operator` selects the node whose checked identity is `quire.op.quantity.convert` and unwraps, so the test fails if no such node exists. For both Equal and NotEqual it asserts `Operands(IneligibleOperator)`, so `UnknownOperator`, `NotApplication`, `OperatorMismatch` or a success would each fail it. The test can fail. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The amended FR-014-AC-2 matrix row says a conversion node is refused as `Operands(IneligibleOperator)` and cites `a_non_equality_operation_refuses_as_an_ineligible_operator`. That is the `integer.lt` case. The test that actually exercises a conversion is `a_quantity_conversion_refuses_as_an_ineligible_operator`. This is a wording fix only: the behaviour is correct and tagged. Fix: cite the conversion test, or both tests. | spec/oracle/matrix/tests.md:18 |

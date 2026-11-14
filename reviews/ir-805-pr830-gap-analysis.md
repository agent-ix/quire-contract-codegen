---
id: SR-17940486709564847047974
title: "IR-805 PR #830 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #830 at 5a16e153, git diff origin/main...HEAD; IR-805 slice E; FR-015-AC-98, -105, -106, -107, -108; tests/it/kani_graph_private.rs, tests/it/kani_obligations_symbolic_graphs.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805 slice E. Plan completion: not assessed. `quoin matrix --json` at 5a16e153 lists
FR-015-AC-98, -105, -106, -107 and -108 as `tagged`. The new binders are
`a_transitivity_claim_with_three_quantifier_bound_operands_is_three_items` (AC-98, -105, -107),
`quantified_claims_the_encoding_cannot_express_settle_unsupported_claim_shape` (AC-108),
`one_node_under_two_clauses_is_two_items_with_different_identities` (AC-107, -100), and three ignored
real-Kani tests (AC-106). For each criterion I compared the evidence with its text. All five stay
PARTIAL, which is correct.

## Verdict

**PASS with findings (no high).** AC-107's "one node under two clauses" clause is now evidenced by a
real QSL package (three declarations, two structures). The test covers two identities, the duplicate
rejection and `MissingClaim` for an absent occurrence. AC-106 gains a healthy multi-reachability
control with a harness mutant and a claim mutant, all three non-vacuous (see SR-17940486709564847047973).
The claim-mutant settlement is honestly `Inconclusive` (Witness cause), not
`ReproducedWithEvaluatedWitness`. The two gaps are both about how much credit the healthy control
earns.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Transitivity of positive-length reachability holds on every graph: it is a theorem of the transitive closure and does not depend on the `{0,3}` draw. So the healthy `Verified` evidences the harness closure, paired with the no-closure mutant, but not the edge domain. AC-106's row credits it as the non-tautological healthy claim "over such an end". Scenario: a regression that collapses the edge draw (degrees forced to 0, or the uniqueness assumption dropped) still verifies healthy transitivity with `object_count == N` covered. Only the mutant's witness assertion shows that at least two edges are drawn, and nothing evidences uniqueness or the upper bound (that is slice C). State in AC-106 that this claim is domain-independent and evidences the closure only. | spec/kani/functional/FR-015-bounded-kani-obligations.md:951; tests/it/kani_graph_private.rs:758-776 |
| FND-002 | medium | AC-106's healthy case requires a `{0,3}` unique and a `{0,3}` non-unique end. Slice E runs only the unique `kids` end. The row's "NOT evidenced for the quantified claim" list names the evaluated-witness settlement, the field-edge claim and the another-field-or-role mutant, but not the missing non-unique healthy control. A reader would take the `{0,3}` healthy-case clause as closed for both ends. Add the non-unique healthy transitivity run, or list it as not evidenced. | spec/kani/functional/FR-015-bounded-kani-obligations.md:951; tests/it/kani_graph_private.rs:758-776 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-106 | examined | With the installed backend, a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover satisfied for one field-edge claim and one relationship-end claim over a `{0,3}` unique and a `{0,3}` non-unique end. Each semantic mutant yields `Falsified` ... and native replay of that same authored clause ... |
| FR-015-AC-107 | examined | Graph claims are located by `state_clause` node id and argument path: one reachability written twice in one condition yields two items at positions 0 and 1; one node under two clauses yields two items with different identities; ... |
| FR-015-AC-98 | examined | A graph claim ... generates a harness when the admitted package gives its covering object type `E` a finite population `[0, N]` ... PARTIAL (IR-805 slice A). |
| FR-015-AC-105 | examined | For a graph claim the harness draws an object count in `[0, N]` of the one covering population ... PARTIAL (IR-805 slice A). |
| FR-015-AC-108 | examined | A graph claim settles by the disposition sequence ... a claim form the encoding cannot express, each settle `unsupported` naming the input. PARTIAL (IR-805 slice A), evidence by branch. |
| FR-015-AC-100 | context_only | For the graph family (AC-98), changing the claim's clause id or argument path ... changes the harness identity ... |

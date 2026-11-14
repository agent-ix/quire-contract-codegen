---
id: SR-17940486709564847047975
title: "IR-805 PR #830 spec-review/base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #830 at 5a16e153; spec/kani/functional/FR-015-bounded-kani-obligations.md (lines 164, 951, 952)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-805 slice E. I reviewed the three spec edits in the diff: the IR-805 status narrative,
the AC-106 status, and the AC-107 status. I checked each against the code and tests on the branch,
and checked that every graph criterion is honestly PARTIAL. Criterion strength was judged manually
(Jev not used). The edited prose contains no pins, hashes or digests.

## Verdict

**PASS with findings (no high).** The status claims are honest. The PR stays PARTIAL. The quantified
claim-mutant settlement is stated as `Inconclusive` with a witness cause, not
`ReproducedWithEvaluatedWitness`. The function-body claim, the field-edge form and slice C are named as
not covered. But the AC-106 row keeps a stale sentence that contradicts the new slice E text, and two
status headers were not updated.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AC-106 row contradicts itself. The new slice E text says multi-reachability "slice E supports" and records the healthy transitivity `Verified`. The unchanged tail still says "Not delivered, so AC-106 stays PARTIAL: ...; and a non-tautological `{0,3}` healthy claim (multi-reachability claims, IR-805 slice E: GraphClaim keyed by node+occurrence)", and the status header still reads "PARTIAL (IR-805 slices A and B)". Scenario: a reader or the gap-analysis cannot tell whether slice E is delivered, and the next status refresh keeps both claims. Remove slice E from the not-delivered list and update the header to slices A, B and E. | spec/kani/functional/FR-015-bounded-kani-obligations.md:951 |
| FND-002 | low | AC-107's status header still reads "PARTIAL (IR-805 slice A)". Its normative text says claims are "located by `state_clause` node id and argument path", but the implementation keys by `(node, occurrence, path)`. AC-100's graph identity list omits the occurrence, even though `one_node_under_two_clauses_is_two_items_with_different_identities` is tagged AC-100 for exactly that identity change. Name the occurrence in AC-107's location key and in AC-100's identity components. | spec/kani/functional/FR-015-bounded-kani-obligations.md:945, 952 |
| FND-003 | low | The narrative is a run-on: "...the wrong-role clause is NOT evidenced) slice E adds the multi-reachability (transitivity) claim..." is missing a separator, so slice E reads as part of AC-106's slice B parenthetical. | spec/kani/functional/FR-015-bounded-kani-obligations.md:164 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015 IR-805 status narrative | examined | Slice A of IR-805 (relationship-end graph claims) is delivered and every graph criterion is PARTIAL with its own stated gap ... slice E adds the multi-reachability (transitivity) claim over a `{0,3}` end with the occurrence-keyed claim (status stated in AC-106 and AC-107), and it still pends slice C ... |
| FR-015-AC-106 | examined | ... NOT evidenced for the quantified claim: QSL settles that replay `Inconclusive` with a witness cause ... so `ReproducedWithEvaluatedWitness` is not reached ... Not delivered, so AC-106 stays PARTIAL: ... and a non-tautological `{0,3}` healthy claim (multi-reachability claims, IR-805 slice E ...) |
| FR-015-AC-107 | examined | Graph claims are located by `state_clause` node id and argument path ... PARTIAL (IR-805 slice A), evidence by clause ... Slice E: a graph claim is keyed by the clause node id and one of that node's own source occurrences ... Not evidenced: a function-body claim ... and the `reaches_field` form (slice D). |
| FR-015-AC-98, -105, -108 | examined | Unchanged; PARTIAL (IR-805 slice A) stays accurate. |
| FR-015-AC-100 | context_only | For the graph family (AC-98), changing the claim's clause id or argument path ... changes the harness identity |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The AC-106 header reads "PARTIAL (IR-805 slices A, B and E)". The not-delivered tail no longer lists slice E; it lists the `{0,3}` degree and target draws, the wrong-role mutant, slice D and slice C. |
| FND-002 | fixed | The AC-107 header reads "PARTIAL (IR-805 slices A and E)" and its key is "node id, the node's source occurrence and argument path". AC-100's graph identity list names "the clause node's source occurrence". |
| FND-003 | fixed | Line 164 now reads "...the wrong-role clause is NOT evidenced); slice E adds...". |

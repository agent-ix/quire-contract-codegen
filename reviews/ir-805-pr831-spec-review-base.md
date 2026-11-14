---
id: SR-17940486709564847047978
title: "IR-805 PR #831 spec-review/base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #831 at 284ced6c; spec/kani/functional/FR-015-bounded-kani-obligations.md (lines 164, 944, 951); spec/kani/matrix/TC-025-bounded-kani-obligations.md (lines 210, 260); spec/kani/matrix/tests.md (line 50)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805 slice C. I reviewed the status-prose edits in FR-015 (the IR-805 narrative, AC-99 and
AC-106), in TC-025 (the graph-family status and the IR-939 paragraph) and in the tests.md row. I
checked each against the code on the branch and against my rerun at the head (16/16 passed).
Criterion strength was judged manually; Jev was not used. The edited prose adds no pins, hashes or
digests, and no requirement statement changed.

## Verdict

**PASS with findings (no high).** AC-106 stays honestly PARTIAL:

- slices A, B, C and E are named;
- the wrong-role mutant is explicitly NOT evidenced;
- the field-edge form and its domain controls are left to slice D;
- the slice C evidence states its own limits (a test harness, not a production generator; the
  solver-chosen Falsified witness; no field population claimed).

The findings are wording and consistency defects only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The tests.md row's status label still reads "Partial (IR-805 slices A and B)" while the row now records slice C as delivered. Slice E is also missing from the label. | spec/kani/matrix/tests.md:50 |
| FND-002 | low | The rewritten TC-025 graph-family sentence is missing a separator ("...the lib tests run outside `make kani-gate`) slice C is delivered..."). It also keeps "AC-107 lacks evidence for the one-node-under-two-clauses clause", which contradicts the AC-107 row, where slice E evidences that clause. The same stale claim is in tests.md:50. The contradiction predates this PR, but it sits on lines this PR rewrote. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:210; spec/kani/matrix/tests.md:50 |
| FND-003 | low | The AC-106 slice C evidence says "The candidate is drawn without any G or D filter; only dormant padding is zeroed". It omits that the carrier also bounds active values (count <= 5, targets/operands < 5, degree <= 4), and it gives no evidence that the compared domains fit that box (see SR-17940486709564847047976 FND-002). | spec/kani/functional/FR-015-bounded-kani-obligations.md:951 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015 IR-805 narrative | examined | "slice C delivers the independent domain conformance and completeness oracle and the three pure-bound mutants for the relationship-end form (status stated in AC-106)" |
| FR-015-AC-99 | examined | "graph proof controls are AC-106 (PARTIAL, IR-805)" |
| FR-015-AC-106 | examined | "PARTIAL (IR-805 slices A, B, C and E) ... Not delivered, so AC-106 stays PARTIAL: a wrong-role mutant ... is NOT evidenced" |
| TC-025 graph-family status | examined | "slice C is delivered for the relationship-end form (the domain oracle, healthy pass, boundary probes and the three pure-bound mutants ...)" |
| TC-025 IR-939 paragraph | examined | "For FR-015-AC-106's separate domain controls (IR-939, slice C of IR-805: delivered for the relationship-end form in `tests/it/kani_graph_domain_private.rs`; the field-edge form is slice D)" |
| tests.md FR-015 graph row | examined | "slice C (domain conformance and completeness oracle, ...) is delivered, and it pends slice D" |

## Dispositions

Round 1, fix commit 5f550fc3.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The tests.md label now reads "Partial (IR-805 slices A, B, C and E)". |
| FND-002 | fixed | TC-025 now has the separator ("kani-gate`); slice C is delivered"). The stale AC-107 claim now reads "AC-107's one-node-under-two-clauses clause is evidenced as its row states" in both TC-025 and tests.md. |
| FND-003 | fixed | AC-106 now states that the candidate "is confined to the carrier's box (count at most 5, operands and targets below 5, degrees at most 4)", and it cites the box test. |

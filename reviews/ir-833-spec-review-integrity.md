---
id: SR-12086
title: "IR-833 integrity review: FR-021 and TC-031 internal consistency after the boundary conversion"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@843588258cc236867caedfc34ef5683ea7b77076 (PR #416); spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-031
    type: references
---

## Summary

Ticket: IR-833. Checked that the converted criteria agree with the unchanged text around them: lists of criterion ids that name execution gates or vectors, mutation rows, the TC-031 step 11 extension, and the FR-021-AC-4 split. FR-014, FR-018, TC-024 and TC-029 edits are internally consistent (the TC-029 step 4 "leg (a)" wording was correctly rewritten once leg (b) was removed, and the title change is mirrored in the tests.md summary row).

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| TC-031 step 11 | examined | Compare outcomes, typed input refusals, charge sequences and consumed counters with an independently constructed shallow equivalent; no comparison with a QSL-owned package runs here (outside CG's boundary, owned by quire-integration IR-842; FR-014-AC-47); ... This is the distinct PLANNED execution gate for AC-2, AC-4, AC-5, AC-8, AC-9, AC-16 through AC-20, AC-26 and AC-31 |
| FR-021 Status :233-237 | examined | AC-2 through AC-5, AC-8, AC-9, AC-14, AC-16 through AC-20 and their TC-031 vectors remain retained as target-neutral expected behavior; their present Contract Runtime `exact` witnesses are historical. |
| FR-021-AC-2 | examined | No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row names agreement of an oracle's outcome, admitted charge sequence or consumed counters with a direct application through a QSL-owned package. |
| FR-021-AC-18 | examined | No test name, trace tag, test output line or doc comment of the function-application corpus, no TC-031 step and no oracle-matrix row names agreement in outcome, refusal or charge order with the QSL-owned application authority. |
| FR-021-AC-4 | examined | The schedule records ordered arguments without inventing a refusal result. Whether an eventual oracle's arity, wrong-value-kind, dangling-reference and unknown-function input refusals equal the QSL-owned ones is outside CG's boundary ... |
| FR-021 mutation rows AC-2, AC-4, AC-18 | examined | FR-021-AC-4: Collapse distinct owner input refusals into one generic cause after the API migration. |
| FR-021 :174-179 | examined | Nested calls retain source operand order and the charge-before-work order of FR-021-AC-5. |
| TC-029 step 4 | examined | ... reading it back out of the generated crate would make the comparison follow the mutation and be vacuous; it is held by the test. |
| FR-021-AC-32 | context_only | preserves AC-2 through AC-5, AC-8, AC-9, AC-14 and AC-16 through AC-20 against the QSL-owned API. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-031 step 11 still names AC-2 and AC-18 (inside "AC-16 through AC-20") as criteria whose PLANNED execution gate it is, and the FR-021 Status paragraph the diff edits still lists AC-2 and AC-18 among the criteria with "their TC-031 vectors". Both are now Inspection boundaries with no vectors and nothing to execute. Drop AC-2 and AC-18 from both lists (for example "AC-4, AC-5, AC-8, AC-9, AC-16, AC-17, AC-19, AC-20, AC-26 and AC-31"). AC-32's list is IR-845's and is left as is. | spec/oracle/matrix/TC-031-function-application-oracles.md:128-130, spec/oracle/functional/FR-021-function-application-oracles.md:236-237 |
| FND-002 | low | FR-021-AC-2 and FR-021-AC-18 are now the same inspection over the same artifacts (function-application corpus tests, TC-031 steps, oracle-matrix rows), differing only in listing "outcome, admitted charge sequence or consumed counters" versus "outcome, refusal or charge order" and "QSL-owned package" versus "QSL-owned application authority". One criterion covering outcome, refusals, charges and counters would do; the other id could be retired as superseded rather than kept as a near-duplicate. | spec/oracle/functional/FR-021-function-application-oracles.md:249, spec/oracle/functional/FR-021-function-application-oracles.md:265 |
| FND-003 | low | TC-031 step 11 now compares the main corpus and the 100,000-node case with "an independently constructed shallow equivalent". Before the edit the shallow equivalent was the comparator for the shallow-body cases beside the QSL-owned package; with the package removed, the step no longer says what the main corpus and the deep case are compared with, or that the comparator is built from the request (FR-021-AC-3). | spec/oracle/matrix/TC-031-function-application-oracles.md:121-124 |

## Verdict

CHANGES REQUESTED (one medium, two low). The FR-021-AC-4 split is structurally sound: the remaining schedule clause is a testable statement and the boundary sentence follows the FR-014-AC-37 pattern (evidence-side consequences are in SR-12087). The :176 edit replacing "the QSL authority's charge-before-work order" with "the charge-before-work order of FR-021-AC-5" is correct and self-contained. The FR-021 mutation rows for AC-2 and AC-18 are rewritten to inspection mutations that would break them.

## Dispositions

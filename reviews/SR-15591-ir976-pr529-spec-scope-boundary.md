---
id: SR-15591
title: "IR-976 PR #529 scope and boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md; spec/oracle/matrix/TC-058-generated-closure-evaluation-corpus.md"
review_set: subset
---

## Summary

Checked CG's producer obligations and its QSL/quire-eval-corpus/quire-eval boundaries against QSL FR-402 at QSL main 61890262. CG owns the export and independent case census; FR-402 owns the case schema, admission, compared record and harness. One staging obligation disagrees with QSL.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-1 and TC-058 permit named generated bodies with `expected: null` before the quire-eval implementation exists, but QSL FR-402's "What CG writes" requires `generated_bodies` empty in that stage. AC-2's generated-body perturbation also needs to be staged after implementation. A null-stage bundle with body names violates its owner contract even if the loader accepts the record. | FR-044-AC-1, FR-044-AC-2, TC-058; QSL FR-402 "What CG writes" |

## Verdict

Blocked on FND-001. The QSL FR-060 T12-A boundary is otherwise respected by locating `compile_package` inside CG's replay module and using `quire-eval-corpus` for the case wire.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At f43e5146, FR-044-AC-1 and TC-058 require `generated_bodies: []` with `expected: null` before implementation. FR-044-AC-4 and TC-058 stage sorted registered-body names and their perturbation with the non-null expectation; AC-2 no longer tests a pre-implementation body-name perturbation. This matches QSL FR-402's "What CG writes" paragraph. |

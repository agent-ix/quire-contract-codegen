---
id: SR-12792
title: IR-845 spec-review/evidence review
type: SpecReview
analysis: evidence
scope: 'agent-ix/quire-contract-codegen; PR #420 own spec delta against #416'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-021
  type: references
---

## Summary

Ticket: IR-845. PASS. Examined the authored Test → Inspection conversion for FR-021-AC-31 and its concrete TC-031 step 11 inspection procedure. No other verification cell changes in this PR. No evidence of QSL application execution is credited.

The mandatory catalog advisor recommends property-based-testing (Test/Property) from rule property_shapes=universal and marks authored Inspection a mismatch. Reviewer judgment confirms Inspection: the revised obligation forbids misleading claims in a finite documentary surface (test names, tags, output text, doc comments, TC-031 and matrix rows); it is no longer an executable 100,000-node oracle behavior property. TC-031 step 11 enumerates that exact surface and rejects treating schedule compilation as execution. The structural universal shape alone does not make generated input testing the appropriate evidence for textual boundary claims. This is an explicit reasoned confirmation of the authored method, not an unexamined advisor disagreement.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-021-AC-31 | examined | spec/oracle/functional/FR-021-function-application-oracles.md | No test name, trace tag, test output line or doc comment of the 100,000-node case, no TC-031 step and no oracle-matrix row claims, or presents as evidence, that a generated oracle compiled, ran on a 512 KiB stack or agreed with a shallow equivalent against a QSL application API; schedule-source compilation (AC-7) is not execution evidence. CG claims no QSL application API (IR-845, FR-014-AC-47). |
| FR-021-AC-31 | examined | spec/oracle/functional/FR-021-function-application-oracles.md | Count schedule-source compilation as execution evidence, or present an owner-path run as evidence without a QSL application API. |
| TC-031 step 11 | examined | spec/oracle/matrix/TC-031-function-application-oracles.md | 11. Inspect (AC-31, FR-014-AC-47): QSL exports no callable application API     through qsl-replay, so CG emits and runs no concrete owner-path oracle     (IR-845). Scan the 100,000-node case's test names, `Trace:` tags, output     lines and doc comments, this procedure's steps and the FR-021 rows of the     oracle matrix; none may claim, or present as evidence, that a generated     oracle compiled, ran or agreed against a QSL application API. Schedule     compilation in step 5 is not execution evidence. Linked-mode admission     (AC-8), no kernel-mode application and no literal outcomes/charge |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for this method and scoped delta. The two original IR-853 deferrals remain deferred.

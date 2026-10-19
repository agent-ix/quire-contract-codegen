---
id: SR-13309
title: "IR-915 PR 463 integrity analysis: excluded accounting-only CG positions"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@fc288843; spec/core/functional/FR-039-contract-package-consumption.md, spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/core/matrix/TC-053-contract-package-consumption.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/core/matrix/tests.md, spec/routed/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-915. PR agent-ix/quire-contract-codegen#463. This analysis checked that ids are
unique and unretired, that each new criterion has a matrix row and a test-case procedure, and
that existing text is consistent with the change.

- `make spec` passes, with one new EARS warning (SR-13308).
- FR-039-AC-9 and FR-019-AC-53 are new ids with no earlier history.
- Each has a tests.md row marked Planned, is listed in its TC's Traces To cell, and has a TC
  procedure: TC-053 step 7 and the TC-030 AC-53 paragraph.
- The TC-030 status text names AC-53 as planned.

## Examined criteria

- FR-039-AC-9 (examined): matrix row `spec/core/matrix/tests.md:27`, TC-053 traces at `:64`, procedure step 7.
- FR-019-AC-53 (examined): matrix row `spec/routed/matrix/tests.md:16`, TC-030 traces at `:59`, procedure paragraph.
- FR-039 Dependencies and relationships (examined).
- FR-039 scalar-context bullet (examined): "The lowering request SHALL place each selected conversion as a context-only root in that driver's contiguous context span, before the next driver item."
- FR-022 Inputs and TC-033 traces (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The PR adds a contract that quire-driver produces, the excluded position defined by quire-driver FR-001-AC-15 and FR-010-AC-4. Neither FR-039's Dependencies section nor its `relationships` names that producer. A later change on either side cannot find the other by its links. Fix: add an `ix://agent-ix/quire-driver/FR-001` (or FR-010) reference to FR-039's Dependencies. | spec/core/functional/FR-039-contract-package-consumption.md:256-266 |
| FND-002 | low | The existing scalar-context bullet was not updated. It still defines a context span as running "before the next driver item", while the new bullet says an excluded position also ends the span. Under FR-022's new wording, an excluded position is not a driver item, so the two texts give different span boundaries. Fix: change line 156 to "before the next driver item or excluded position". | spec/core/functional/FR-039-contract-package-consumption.md:154-156,93-95 |

## Verdict

**PASS with findings.** The structure is sound: fresh ids, matrix rows truthfully Planned, and
TC traces and procedures added. Two low consistency fixes remain.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | f9bf1929: FR-039 frontmatter adds `references` edges to `ix://agent-ix/quire-driver/FR-001` and `FR-010`. Its Dependencies section adds that quire-driver FR-001 and FR-010 produce the typed lowering-position sequence and preserve excluded positions. |
| FND-002 | fixed | f9bf1929: FR-039:166-167 now reads "in that bound driver item's contiguous context span, before the next bound driver item or excluded position", which agrees with the new span rule. |

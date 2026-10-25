---
id: SR-35012
title: "Integrity review of IR-1007 CG PR 566 (FR-021 re-point to QSL FR-485)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-021-function-application-oracles.md; spec/oracle/matrix/TC-031-function-application-oracles.md; spec/oracle/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-1007. PR agent-ix/quire-contract-codegen#566 at fd2a03a69a75, diff
only. Completeness, consistency and atomicity pass over the changed FR-021
statements, ACs, TC-031 steps and matrix rows.

Consistent: the matrix rows, TC-031 summary row, FR-021 Status and the AC
PLANNED markers agree on which portions are pending IR-1007/IR-595. The
seven acquisition inputs match FR-485's `acquire_checked_closure` signature.
The root-location claim in AC-17 / TC-031 step 6 matches FR-262-AC-13 and
FR-485-AC-6. The owner-plus-wire-id lookup matches FR-485 behaviour 3. The
same-leaf comparison is correctly kept separate from IR-842 parity. The
prior reviewer's FND-001 (TC-031 step 12) is fixed on this head.

Defects: one new SHALL with no AC or test step, and one narrowed prohibition
in AC-16.

## Scope examined

- FR-021 Inputs, new bullet (lines 79-86): examined.
- FR-021-AC-16 (head vs origin/main text): examined.
- FR-021-AC-1..5, 7..10, 14, 17, 19, 20, 26, 31, 32 against tests.md rows and TC-031 steps 6, 7, 11, 12, 12b: examined; consistent apart from the findings below.
- FR-021-AC-24 ("No new refusal variant is added"): context only.
- QSL FR-485 behaviour 3 (lookup by owner package id and wire node id): context only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new Inputs SHALL ("The IR-595 consumer shall bind the acquired QSL callable owner and wire id to the requested function identity before registration or application; a mismatch refuses that item rather than running another callable") has no AC, no TC-031 step and no named refusal; AC-24 also states "No new refusal variant is added". The spec also leaves open how CG's requested identity (Contract IR package id, V2 function node id) maps to FR-485's `DigestRecord` owner id and `WireNodeId`. Add an AC and a 12b step for the mismatch refusal and name the identity mapping. | spec/oracle/functional/FR-021-function-application-oracles.md:79-86 |
| FND-002 | low | FR-021-AC-16's second sentence was narrowed. It went from "No oracle makes a claim that those fields are populated" to "No schedule-only oracle claims those fields are populated; only an executed FR-485 session can support that claim". The change is plausibly intended now that a concrete FR-485 oracle can return location and losses. But the concrete-oracle half now has no PLANNED marker, no TC-031 step and no matrix note, while the row stays ✅ Covered. Either restore the all-oracle prohibition until IR-595, or mark the concrete half Planned. | spec/oracle/functional/FR-021-function-application-oracles.md:279; spec/oracle/matrix/tests.md |

## Verdict

FND-001 needs a fix in this PR (a SHALL with no AC). FND-002 is a low
judgement call: record a disposition or adjust the wording.

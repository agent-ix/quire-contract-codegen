---
id: SR-2026110283823
title: "IR-1295 PR 721 specification integrity analysis"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #721; spec/core/functional/FR-005-cli-conformance.md; spec/core/matrix/TC-007-cross-backend-parity.md; FR-005-AC-3/4; TC-007"
review_set: subset
---

## Summary

Ticket: IR-1295. The changed TC-007 procedure describes a named mutant discrepancy, four adverse register joins, and a register that rejects stale rows. I checked whether those claims can hold together in a normal run and the mutant control.

## Verdict

Changes requested: the mutant control needs an explicit temporary register state. A permanent mutant row conflicts with the rule that a row absent from the measured normal discrepancy list fails.

## Examined

- FR-005-AC-3 and AC-4: differential-row cardinality and the still-open broad qualification.
- TC-007 step 4: named mutant discrepancy and missing, duplicate, bad-fixture, and empty-disposition controls.
- TC-007 register: authoritative table, exact key, stale-row refusal, and empty-register completion rule.
- TC-007 Expected Results: mutant and adverse controls fail with case and pair.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-007 requires the mutant to have a register row that adverse controls remove or duplicate, while the authoritative register rejects any row not measured in the normal run. A permanent mutant row makes the normal run fail; with no row, the remove and duplicate controls lack a starting state. Specify a temporary seeded register input for the mutant controls, or another coherent state that keeps the authoritative normal register free of stale rows. | spec/core/matrix/TC-007-cross-backend-parity.md:45-62 |

## Dispositions

Round 1 at 5a0a88cf02ff3d1ae40dc99aa6821be2ff69eeb1: PASS. TC-007 now uses a temporary in-memory register input seeded with a real fixture row for the mutant. The normal authoritative table rejects that mutant row as stale and is never modified by the control.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The seeded temporary register and its four adverse edits are distinct from the authoritative normal register in TC-007 step 4 and the register contract. |

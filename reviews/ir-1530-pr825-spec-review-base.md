---
id: SR-17940486709564847047963
title: "IR-1530 PR #825 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #825 at 212a7dfd, git diff origin/main...HEAD; IR-1530; spec/kani/functional/FR-056-kani-tractability-refusal.md, spec/kani/matrix/TC-070-kani-tractability-refusal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: reviews
---

## Summary

Ticket: IR-1530. This reviews the two FR-056 sentence edits (Description: the embedded rows; Status: "an add
from 2 bits") and TC-070 step 14, which this PR leaves unchanged, against the new two-row table.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | FR-056 Description now says "CG embeds the measured general-route rows, add from 2 bits and sub from 4 bits ... Widths below that row's `max_operand_bits` on the general route are unmeasured". With two rows, "that row's" has no single referent; say "below a row's `max_operand_bits` (2 for add, 4 for sub)". | spec/kani/functional/FR-056-kani-tractability-refusal.md:44-46 |
| FND-002 | low | TC-070 step 14 still says "assert general-route features at 4 bits are refused, native-route features at the same width run, and a narrower general-route width runs (FR-056-AC-15)". With the add row at 2 bits, a 3-bit add (narrower than 4) is now refused. The test uses 1 bit for add and 3 bits for sub. Name the per-row thresholds in the step. | spec/kani/matrix/TC-070-kani-tractability-refusal.md:71-72 |

## Verdict

Wording only. Neither finding changes behaviour, and both fit in this PR's fix round.

## Dispositions

Round 1, checked against d1c36471.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | FR-056 Description: "Widths below a row's `max_operand_bits` (2 for add, 4 for sub) on the general route are unmeasured". |
| FND-002 | fixed | TC-070 step 14 now asserts add refused at 2, 3 and 4 bits and run at 1, sub refused at 4 and run at 3, and native features at the same widths run; the test adds the native add assertions at 2, 3 and 4 bits. |

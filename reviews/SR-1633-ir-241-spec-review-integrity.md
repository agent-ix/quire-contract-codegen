---
id: SR-1633
title: "IR-241 TC-039 status change spec review (integrity)"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen#295; spec/kani/matrix/TC-039-bounded-proof-ceilings.md (changed); spec/kani/matrix/tests.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md (unchanged context)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-039
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---

# SR-1633: IR-241 TC-039 status change spec review (integrity)

## Summary

Ticket: IR-241. PR: quire-contract-codegen#295. This integrity analysis checks whether the
TC-039 Status change leaves the documents that state FR-028's delivery state consistent with
each other. It does not: two unchanged documents still describe the pre-PR state.

## Method

The new TC-039 Status was compared with the matrix index rows for FR-028 and TC-039 in
`spec/kani/matrix/tests.md`. It was also compared with FR-028's Description and its AC-21 row.
Each claim was cross-checked against the code: `KaniInconclusiveReason::MemoryExhausted`
exists, and the slice criteria are tagged in `quire matrix`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The matrix index still reads `FR-028-AC-1 through FR-028-AC-9 \| TC-039 \| Planned`, and says "FR-028-AC-3 stays planned and unbacked until the `MemoryExhausted` variant exists". Both contradict the new TC-039 Status (AC-1 to AC-4 implemented) and the code (the variant exists, and AC-3 is tagged). | spec/kani/matrix/tests.md:40, spec/kani/matrix/tests.md:42, spec/kani/matrix/TC-039-bounded-proof-ceilings.md:144-148 |
| FND-002 | low | FR-028-AC-21's row still ends "PLANNED (IR-241)", and FR-028's Description lists AC-13 to AC-24 as planned. TC-039 Status says the process-tree part of AC-21 is implemented. Neither document names which part of AC-21 remains: the limit-only stand-in of step 19, and the refinement runs that "every run" will cover. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:197, spec/kani/functional/FR-028-bounded-proof-ceilings.md:30-31, spec/kani/matrix/TC-039-bounded-proof-ceilings.md:171 |

### Failure scenarios

- FND-001: a planner reading the matrix index after merge sees AC-1 to AC-4 as planned and
  AC-3 as blocked on a variant that already exists. That reader re-plans delivered work.
- FND-002: a reader of FR-028 alone concludes AC-21 has no implementation. A reader of TC-039
  alone cannot tell what is left of AC-21.

## Verdict

**Changes requested (FND-001).**

- FND-001 fix: split the FR-028 row of `tests.md` into AC-1 to AC-4 (partially covered, citing
  TC-039 Status) and AC-5 to AC-9 (planned). Drop the stale "FR-028-AC-3 stays planned"
  sentence, and mark AC-21 partial.
- FND-002 fix: name the remaining part of AC-21 in one sentence, either in TC-039 Status or
  beside the AC-21 marker.

Both are spec-text-only edits. No requirement statement needs to change.

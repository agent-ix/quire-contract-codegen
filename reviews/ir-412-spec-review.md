---
id: "SR-632"
title: "IR-412 slice 1 spec review: FR-015-AC-26 to AC-28, TC-025 state clauses, matrix row"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@01204aba72480578167cc3510ba13a535083f39c (disposition round 1 at fea49c75bfdf51a000b042dfa2735cb7cd4300bd); spec/functional/complete-v1/FR-015-bounded-kani-obligations.md, spec/test/complete-v1/TC-025-bounded-kani-obligations.md, spec/test-matrix.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

# SR-632: IR-412 slice 1 spec review

## Summary

Ticket: IR-412 (slice 1). PR: agent-ix/quire-contract-codegen#203, head 01204ab (rebased on main bb8523f; first reviewed at 7fd5a7b). Method:
spec-review, with the EARS and integrity checks folded in. `make spec` passes as part of `make ci`.

## Method

I read the three new FR-015 acceptance criteria, the TC-025 procedure step 8, the TC-025 Blocked
note and the matrix row, and compared them with the implementation and with each other.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-015-AC-26 is compound. It bundles two harnesses, the cover count, identity scoping, range assumptions, the contract assertion and the frame assertion into one criterion. Each can fail on its own, so a failing test cannot say which part broke. | spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:185 |
| FND-002 | low | FR-015-AC-28 is compound. It holds six separate backend outcomes and a replay claim, spread across three tests. | spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:187 |
| FND-003 | low | FR-015-AC-26's "every state field the IR bounds" does not say how a caller's Rust state field maps to an IR object member (by name). That mapping is what decides which fields are assumed in range. | spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:185 |

## Verdict

Approve with low findings. The criteria agree with the code and with TC-025 step 8. The
rewritten Blocked note keeps FR-025's clause-negotiation `unsupported` accounting separate from
the new generator. That is consistent with the code, where the clause renderer still refuses a
frame (`FrameNotClauseRendered`). The completeness of the matrix row is covered in SR-631
FND-001.

## Dispositions

Round 1 at fea49c7. FR-015-AC-26 to AC-28 are split into AC-26 to AC-32, one outcome each. TC-025
and the matrix row follow the split.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 4af93a1 |
| FND-002 | fixed | 4af93a1 |
| FND-003 | fixed | 4af93a1 |

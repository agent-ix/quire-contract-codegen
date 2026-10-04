---
id: "SR-1456"
title: "CG PR 267 spec review (criterion strength): changed ACs of FR-029 and FR-030"
type: SpecReview
analysis: criterion-strength
review_set: subset
date: "2026-10-04"
scope: "agent-ix/quire-contract-codegen@f67151a2ee745368bce77d9de2538e6ea88a4ef5; spec/kani/functional/FR-029-run-outcome-terminal-record.md (AC-14), spec/kani/functional/FR-030-ir-outcome-terminal-map.md (AC-1, AC-2, AC-13), spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
---

# SR-1456: CG PR 267 spec review (criterion strength)

## Summary

Ticket: IR-465. Each changed AC was judged on whether a wrong implementation could fail it, and
whether it is true across the inputs it quantifies over. Spec text was judged against QSL
`DependencyInput::new` as merged.

- FR-030-AC-1 is strong, now with no exclusion.
- FR-029-AC-14 and FR-030-AC-13 can fail: a test can match
  `DependencyLockError::Input(DuplicateIdentity)` and the `invalid_package` code. TC-040 step 13
  and TC-041 step 12 exercise them.
- FR-030-AC-2 asserts both cause and code, but its code sources are not in the map's input. That
  is SR-1453 FND-001 and is not repeated here.

## Verdict

One medium finding, on the first-fault reading of the two repeated-identity ACs.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-029-AC-14 and FR-030-AC-13 say, without condition, that a lock selecting one identity twice "is refused by QSL's `DependencyInput::new` as `DuplicateIdentity` (`invalid_package`)". Deleting CG's pre-check changes which error wins when a lock has more than one defect. Today CG's pre-check runs over the whole sorted list first, so a duplicate always beats an empty identity or a shared owner. After the change, QSL's `DependencyInput::new` refuses "in supply order", library by library. It checks empty identity, then duplicate identity, then shared owner, over CG's identity-sorted list. A lock that also holds an empty identity, which sorts first, is refused `EmptyIdentity` with `invalid_identifier`. A lock where an earlier library shares an owner with the first of the duplicate pair is refused `SharedOwner`, which has the same code but a different variant. As written, the ACs are false for those locks. A fixture with only the duplicate passes either way, so the test does not catch this. Fix: scope both ACs to a lock whose only dependency-input defect is the repeated identity, and state once, in FR-029 or FR-016, that when defects coexist the first refusal is QSL's, in `DependencyInput::new` order over the identity-ascending supply. The sort does not mask the duplicate: QSL detects it through its held map in any order. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:169; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:131 |

## Dispositions

Round 1, reviewed at 69708045e4fcebbc491d3d6c8d9a3c391bc5bd30 (fix commit 6970804 on top of f67151a).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 69708045e4fcebbc491d3d6c8d9a3c391bc5bd30: FR-016-AC-24, FR-029-AC-14 and FR-030-AC-13 are each limited to "a lock whose only defect is a repeated library identity". TC-040 step 13 and TC-041 step 12 use the same scope. The precedence is stated in TC-026 Status, FR-030 Status and AD-003 R-Q1: supply order, and for each library an empty identity, then a repeated identity, then a shared source owner. That matches QSL `DependencyInput::new` as merged, which checks per library, in order, `LibraryName::new` (empty identity), then the held-map duplicate, then `same_owner`. Each limited AC can fail: a test matches `Input(DuplicateIdentity)` and the code. |

---
id: SR-14729
title: 'CG PR 491 spec-review/base: per-step refinement record identity'
type: SpecReview
analysis: base
review_set: subset
scope: 'agent-ix/quire-contract-codegen; spec/assurance/AD-003-evidence-chain.md,
  spec/kani/functional/FR-015-bounded-kani-obligations.md (PR #491 head 0c6bf9f6)'
relationships:
- target: ix://agent-ix/quire-contract-codegen/AD-003
  type: reviews
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: reviews
---

# CG PR 491 spec-review/base: per-step refinement record identity

## Summary

Diff scope: AD-003 E-1 per-step records and FR-015-AC-120 through AC-122 at head 0c6bf9f6. QSL ADR-013 PR #1157 at da13209b7 is open and was used as pending cross-repository context. Its QSV member list (refinement, obligation, hypotheses) agrees with CG.

## Scope examined

- FR-015-AC-120: spec/kani/functional/FR-015-bounded-kani-obligations.md
- FR-015-AC-121: spec/kani/functional/FR-015-bounded-kani-obligations.md
- FR-015-AC-122: spec/kani/functional/FR-015-bounded-kani-obligations.md
- AD-003 E-1 per-step refinement records: spec/assurance/AD-003-evidence-chain.md

## Verdict

FAIL: AC-120 has no valid refinement-only identity comparison under the stated owner refusal; clarify the test domain before implementation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-120 demands a refinement-only identity change for an input that owner validation refuses before identity exists; clarify raw-record hashing or the valid-owner comparison. | FR-015-AC-120; FR-015-AC-122; AD-003 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At 7f7619ba AC-120 compares two independently valid selected owners with matching occurrences, states that a mismatched owner has no identity, and places identity minting after owner and occurrence validation. AD-003 states the same order. |

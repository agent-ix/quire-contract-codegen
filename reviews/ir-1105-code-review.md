---
id: SR-645001
title: "IR-1105 CG #645 scalar receipt code review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #645 head 0e848e23f6330c8f5ab1fa750760b6bd64ec1e6d against fix/ir1105-lock-replay-removal; FR-017-AC-29, AC-41 through AC-50"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

## Summary

Reviewed CG #645 exact head 0e848e23f6330c8f5ab1fa750760b6bd64ec1e6d against its #638 base and merged CG #644 specification. Scope: `src/kani/run/scalar.rs`, `src/kani/test_support.rs`, `src/lib.rs`, and the three changed scalar integration test files. The independent Claude Opus review saw the same diff; its speculative concerns about old specification text, UTF-8 checks, path-set checks, snapshot cleanup and other dependencies were checked against current source and merged spec. No build or gate result is claimed.

## Verdict

CONDITIONAL. Fix the orphaned comment before merge. Removal of the scalar digest API, preservation of ten retained byte entries, typed snapshot checks and direct artifact parity match the merged criteria reviewed here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Deleting the shared digest module left its comment above `scalar_source_fixture`; the comment now falsely says this fixture is an oracle shared with the integration binary and never calls canonicalization. Remove the orphaned comment. | src/kani/test_support.rs:199 |

## Coverage

FR-017-AC-29, AC-41 through AC-50 were examined against merged #644 spec; AC-43 still has native-result tests, AC-45 has compile-fail and real receipt controls, and AC-49 has direct original artifact byte comparisons. The source adds no vendored material or duplicated upstream type. Full tests are deferred to the code lead's final reviewed head gate. No applicable AssuranceProfile was found under the scoped `spec/` tree. The PR is stacked, so its worktree's pre-#644 spec is not the normative version used for this review.

---
id: SR-15417
title: "IR-962 seeded native agreement acceptance gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@6c7dafe2bc0659771be68555faa6f46d78393de8; spec/oracle/functional/FR-014-exact-scalar-oracles.md; spec/oracle/functional/FR-018-composite-equality-oracles.md; spec/oracle/functional/FR-021-function-application-oracles.md; tests/it/scratch_crate.rs; tests/it/exact_scalar_agreement.rs; tests/it/composite_equality_generation.rs; FR-014-AC-6; FR-018-AC-2,12; FR-021-AC-2"
review_set: subset
---

## Summary

Ticket: IR-962. Diff-scoped acceptance-chain review of the native agreement lock setup. The computed Quire matrix binds FR-014-AC-6 to 17 tests, FR-018-AC-2 to 19, FR-018-AC-12 to two, and FR-021-AC-2 to two. The changed call sites remain inside already tagged tests; the support helper introduces no product behavior and needs no new acceptance criterion. The ticket's root-cohort claim is checked structurally: the scratch lock starts from the root Cargo.lock, offline metadata adapts the scratch graph, and every resolved Git source must match a source in that root lock before the locked/offline native command executes. The code that compares native results and fails named mutants is unchanged.

## Verdict

**PASS** for diff-scoped traceability and control retention. No new untagged product criterion, source stub, test skip, or assertion removal was found. This is a planless PR-diff audit; it does not claim repository-wide completeness. Author focused-control logs were unavailable locally, so this review does not certify their run counts or outcomes; final-head CI and Kani remain the PR owner's gate.

## Coverage

- FR-014-AC-6, FR-018-AC-2, FR-018-AC-12, and FR-021-AC-2 (examined): matrix binders still reach the affected scratch-crate setup and native commands; none of their trace tags changed.
- Reverse gap (examined): the only new function is test support for reproducible dependency setup, owned by IR-962 and invoked by both affected call sites; no production symbol was added.
- Stub and inflation scan (examined): no new `todo!`, `unimplemented!`, ignored test, fake native result, removed assertion, or substituted test double appears in the three-file diff.
- Plan completion: not assessed. Optional repository-wide semantic review: not requested. This is a PR-diff audit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No gaps found (placeholder) | - |

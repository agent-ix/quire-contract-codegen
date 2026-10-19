---
id: SR-13102
title: "Code review — repeated backend identity refusal"
type: SpecReview
analysis: code-review
scope: "quire-contract-codegen PR #451; src/routed/capability.rs, tests/it/capability_settlement.rs, spec/routed/matrix/tests.md, .github/workflows/ci.yml"
review_set: subset
---

## Summary

Reviewed the PR diff against FR-019-AC-10, the referenced FR-290 refusal vocabulary, repository Rust conventions, and the Rust review checklist. The envelope refuses any repeated exact backend identity before item settlement, including byte-equal descriptors, and selects a stable repeated identity when several exist. The CI workflow is unchanged.

## Verdict

**PASS** — No findings in the changed scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

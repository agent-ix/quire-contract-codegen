---
id: SR-8381
title: "Gap analysis of IR-764 fixture admission"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@283faea800d3f8a1287787498aeec3cd713bd0c7; spec/core/functional/FR-037-fixture-identity.md, spec/core/matrix/TC-052-fixture-identity.md, changed src/ and tests/ Rust files"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-037", type: references }
---

## Summary

Compared the PR's changed fixture behavior and tests with IR-764's acceptance and FR-037/TC-052. Positive corpus admission and measured malformed-shape refusals have direct reader assertions, but the required stale application key and unchanged-key body mutation controls have no direct reader test.

## Verdict

**FAIL** — the missing negative controls leave a required rejection path unverified.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | No direct reader assertions for a stale application key and an application body mutation under its original key | FR-037-AC-4; tests/it/exact_scalar_generation.rs:287; tests/it/composite_equality_generation.rs:78 | correct-requirement-no-evidence |

## Coverage

- Reconciliation: quoin matrix 0.28.3 over the frozen repository, scoped interpretation for FR-037. AC-1 is tagged by 13 tests; AC-2, AC-3, and AC-4 are untagged; AC-5 is inspection-only. Other repository criteria were outside this PR review and their statuses are not a verdict on this PR.
- Plan completion: not assessed
- The new TC-052 refusal tests mutate a noncanonical bound or sequence shape. Those tests preserve useful malformed-shape controls but do not submit an otherwise valid package with an incorrect application ID or a changed application body under its old ID. The shared `resolve_wire` helper repairs stale keys before returning; its reader call cannot serve as the negative oracle.
- Reverse-gap/stub review of changed files found no additional new gap. Semantic intent was checked for IR-764 acceptance as requested in this PR review; no repository-wide semantic pass was run.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5601cee5cacbd1bab5a2bd321856876bd3111671 — `tc_052_reader_refuses_stale_application_key_in_admitted_positive_wire` changes the key on an admitted wire and asserts `StaleNodeKey` with the prior admitted ID; `tc_052_reader_refuses_application_body_mutation_under_prior_key` changes the literal without rekeying and asserts the reader's expected ID equals an independently derived public candidate. Both call `read_mutated_wire`, which only refreshes the package identity before its direct `CheckedPackageV2::read`. |

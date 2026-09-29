---
id: SR-046
title: "Gap analysis — quire-contract-codegen PR #190 remove Kani digests and version stamp"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@90a1091754f77a74245d535b16b9262a0080fd0b; spec/test-matrix.md, spec/functional/**, tests/, src/"
review_set: subset
---

## Summary

This compares requirement coverage at the PR head with `origin/main` 6f4beea. It runs `quire coverage --scope . --strict` on both and diffs the output. No ticket id is derivable. The PR changes no `spec/` file. No requirement text named the generated-file notice or the Kani executable or options digests, so no AC loses its subject.

## Verdict

**PASS**. The coverage output is identical on main and head: both EXIT=1 with 31 unbacked rows and 0 contradicted statuses, which predate this PR. The only difference is one pre-existing `oracle-resembles-implementation` note whose line moved from `tests/exact_scalar_support/package.rs:3768` to `:3774`, because the fixture grew six lines. No traced test was deleted or untagged. tc_001 (tests/it/oracle_generation.rs) and the tc_025 tests (tests/it/kani_obligations.rs) keep their trace tags and their assertions.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

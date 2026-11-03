---
id: SR-2026110283841
title: "IR-724 PR #732 FR-036 coverage target specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #732; IR-724; spec/evidence/matrix/tests.md:21; TM-007 FR-036 coverage row"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-036
    type: reviews
---

## Summary

Reviewed the TestMatrix row identifier, its seven referenced FR-036 criteria, TC-051, and the unchanged Planned status. Quire previously parsed the Markdown link spelling in the Functional Req cell as the literal target `[FR-036](../functional/FR-036-source-obligation-census.md)`. On this PR head, `quire coverage --scope . --strict --json` parses the same row as `FR-036` with target ids `FR-036` and `TC-051`. The row still has no evidence credit: strict coverage reports 272 unbacked rows and zero status lies before and after this one-cell correction. `quire validate --scope . spec/evidence/matrix/tests.md` and `git diff --check` pass. No applicable `AssuranceProfile` exists under `spec/`.

## Verdict

**PASS.** The bare requirement ID resolves the matrix target without changing the planned evidence claim or the test procedure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| TM-007 FR-036 row | examined | Functional Req `FR-036`; acceptance criteria `FR-036-AC-1` through `FR-036-AC-7`; test case `TC-051`; status Planned under IR-740 and gated on IR-759. |
| FR-036 | context_only | A source-obligation census exports complete selected obligation, candidate, and mapping-gap inventories before generation or execution. |
| TC-051 | context_only | Planned procedures use genuine typed owner inventories and reviewed IR-759 correspondence; no executed test or proof baseline is claimed. |

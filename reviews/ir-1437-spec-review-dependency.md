---
id: SR-17940486709564847047752
title: "IR-1437 PR #810 FR-055 dependency review"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen PR #810; IR-1437; spec/kani/functional/FR-055-kani-mutations-target.md; spec/kani/matrix/TC-069-kani-mutations-target.md; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
---

## Summary

Ticket: IR-1437. Dependency lens over the new `relationships:` edges. FR-055 has `satisfies`
StR-001 and `depends_on` FR-017 and NFR-006. TC-069 `verifies` FR-055. NFR-006 now has a
statement that names FR-055, but it has no edge to FR-055 and its Dependencies section does not
list it. FR-055 names FR-041 and FR-046 to FR-054 as the source of its mutants, but has no edge to
any of them.

## Verdict

**PASS with one low finding.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | NFR-006's new statement requires FR-055, but NFR-006 has no relationship edge to FR-055 and its Dependencies omit it. FR-055 takes its mutant set from FR-041 and FR-046 to FR-054 but declares no edge to them, so the source of "every generated harness's mutant" is not traceable | NFR-006, FR-055, FR-041 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-055 | examined | relationships: satisfies StR-001; depends_on FR-017; depends_on NFR-006. Body: mutant controls of FR-041 and FR-046 to FR-054. |
| TC-069 | examined | relationships: verifies FR-055. |
| NFR-006 | examined | relationships: constrains FR-015, FR-017, FR-031, FR-041; Downstream: TC-045. Statement: `make ci` shall depend on `kani-mutations` (FR-055). |

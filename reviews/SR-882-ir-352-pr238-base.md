---
id: "SR-882"
title: "CG PR 238 spec review: TC-031 and Test Matrix status flips for FR-021-AC-19 to AC-21"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@5cb264db4df4ba16290d19236369e530d5debb62; spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md (diff origin/main...HEAD, base 80f4785)"
---

# SR-882: CG PR 238 spec review

## Summary

Ticket: IR-352. The spec diff changes status text only. It adds or edits no requirement, AC,
test procedure step or relationship. So only the base spec review applies. The EARS,
integrity, object and dependency sub-analyses have nothing in scope.

What I checked:

- `tests.md`: the row `FR-021 | FR-021-AC-19 through FR-021-AC-21 | TC-031` goes from Planned,
  with a forward-looking note, to `✅ Covered`. The neighbouring rows for FR-021-AC-15 and
  FR-021-AC-18 stay Planned with their reasons. The prose under the table still describes FR-018
  only, so it is unaffected.
- TC-031: the Description now says only the FR-021-AC-18 authority leg is Planned. Step 8 loses
  its trailing Planned marker, and its procedure text is unchanged. Expected Results drops
  "(all 🚧 Planned)". Step 6 (line 67) and Expected Results (line 113) still say Planned, and
  both are about AC-18. Nothing left in the file calls AC-19 to AC-21 Planned.
- Consistency with code: SR-881 confirms that each of the three ACs has a passing test that I
  saw fail under mutation.
- `quire validate` over `spec/**` passed in the PR's gate log (`cg-352-ci.log`).

## Verdict

The status flips are accurate and consistent between the TC file and the matrix. Clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

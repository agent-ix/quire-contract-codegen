---
id: SR-1761
title: "IR-639 guardian-test-support spec review (scope boundary)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@7742a43f62fc3feddde61a529a2bab37b4d0a67b; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md, spec/kani/matrix/tests.md, spec/tests.md (PR #305 diff against main 28553daeb1e9cfd88bb6620125bb1df27273c68e)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1761: IR-639 guardian-test-support spec review, scope boundary

## Summary

Ticket: IR-639. PR: quire-contract-codegen#305. Reviewer: claude-opus-5-5, session
9072f908-e626-4176-ae33-69e582990445. I checked which system owns each new obligation: the CG
library, the CG helper, the CG test suite and CG documentation, or the external production driver.
I also checked that every CG criterion can be satisfied by CG's own TC-049. One medium finding.

## Method

I classified each clause of FR-034-AC-27 to AC-30, the new FR-034 section and TC-049 steps 13
and 14 by the actor that must act. I searched `spec/`, `README.md` and `docs/` for a definition
of "production driver" or "production profile". The only occurrences are the two files this PR
changes. Scope units examined: FR-034-AC-27, AC-28, AC-29 and AC-30, the new FR-034 section,
and TC-049 steps 13 and 14.

## Verdict

**FAIL: one medium finding.** AC-27, AC-28 and AC-29 are allocated to CG's library, helper and
tests, which CG can satisfy. The PR changes no product Rust code, manifest, CI workflow or driver
repository, and it claims no delivered coverage.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034-AC-30 is a CG acceptance criterion with Test verification, but its subject is "production-driver dependency-edge verification" in "any production profile". That actor, its repository and "production profile" are defined nowhere in the spec. TC-049 step 14 itself says a CG-only check can never pass it. AC-30 therefore stays permanently unsatisfiable in CG's matrix, and it has no owning requirement or ticket in the repository that must act. Allocate it to the driver's own requirement or ticket and cite that here as a dependency. Alternatively, restate the CG-side obligation (for example, CG documents the feature as test-only and supplies a check the driver can run) and keep only that in AC-30. | spec/kani/functional/FR-034-caller-death-ownership.md:218-222, spec/kani/functional/FR-034-caller-death-ownership.md:273, spec/kani/matrix/TC-049-caller-death-ownership.md:149-157 |

## Dispositions

Round 1, reviewed at ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ef9ee8ec51b81c4f9a967c6ea738e16cc9e4528d: AC-30 is now a CG publication/allocation criterion verified by Inspection. The downstream dependency-edge assertion is allocated to IR-649 (quire-driver, exists in Backlog) and kept as a separate pending gate in the Dependencies section and the matrix. |

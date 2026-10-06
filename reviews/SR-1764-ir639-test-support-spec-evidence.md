---
id: SR-1764
title: "IR-639 guardian-test-support spec review (evidence)"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@7742a43f62fc3feddde61a529a2bab37b4d0a67b; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md, spec/kani/matrix/tests.md (PR #305 diff against main 28553daeb1e9cfd88bb6620125bb1df27273c68e)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1764: IR-639 guardian-test-support spec review, evidence

## Summary

Ticket: IR-639. PR: quire-contract-codegen#305. Reviewer: claude-opus-5-5, session
9072f908-e626-4176-ae33-69e582990445. I compared each new or changed criterion's declared
verification method with the evidence TC-049 actually prescribes for it. One low finding.

## Method

For FR-034-AC-14, AC-23, AC-24 and AC-27 to AC-30, I matched the Verification column against
TC-049 steps 1, 12, 13 and 14 and the Expected Results row. Runtime assertions and compile-fail
checks count as Test. Reading source structure or documentation counts as Inspection. Scope units
examined: FR-034-AC-14, AC-23, AC-24, AC-27, AC-28, AC-29 and AC-30, and TC-049 steps 12 to 14.

## Verdict

**FAIL: one low finding.** The following match:
- AC-14 (documentation) is Inspection.
- AC-29's feature-off absence uses a compile-fail Test.
- AC-24's oracle is a runtime Test.
- The matrix row claims no executed evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-034-AC-27 ("No lease, process-ownership handle, public cancellation entry or cleanup-deferring callback is exported") and FR-034-AC-28 ("cannot ... alter a production-stage branch") are structural properties of the source. A runtime test cannot fail on them. TC-049 step 13 verifies them by inspection ("Inspect unconditional shared production stage/cleanup paths"), yet both criteria declare Test only. Declare Test and Inspection for these clauses, or name a mechanical check that counts as Test, such as a public-API surface assertion or a syn-based source check. | spec/kani/functional/FR-034-caller-death-ownership.md:270-271, spec/kani/matrix/TC-049-caller-death-ownership.md:144-148 |

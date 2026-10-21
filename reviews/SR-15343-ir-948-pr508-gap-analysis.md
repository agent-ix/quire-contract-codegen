---
id: SR-15343
title: "IR-948 PR 508 V2 precondition gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@57da5d7fe68fbda8fdcd1632334ab919d6cc03fe; spec/kani/functional/FR-015-bounded-kani-obligations.md; spec/kani/matrix/TC-025-bounded-kani-obligations.md; src/kani/generate/{negotiate,v2_clause}.rs; tests/it/kani_obligations_state_frame.rs; FR-015-AC-38,39,40,42,46,47"
review_set: subset
---

## Summary

Ticket: IR-948. Diff-scoped acceptance chain at PR 508 head 57da5d7fe68fbda8fdcd1632334ab919d6cc03fe. This is the parameter precondition slice of the planned V2 clause input. The test for FR-015-AC-47 is tagged by criterion id; the generation and real Kani controls use TC-025 while the broader AC-38/39/40/42/46 remain planned for later V2 clause slices. The previously measured impossible-precondition memory limit is retained under IR-705 and is not claimed by this PR.

## Verdict

**PASS** for the stated slice; the broader planned criteria remain open. No new unowned behavior, stub or hollow positive test was found in the changed path.

## Coverage

- FR-015-AC-38 (examined): a precondition node produces its own V2 harness and records its role; postcondition and invariant remain planned.
- FR-015-AC-39 (examined): generated source assumes the admitted 0..=1000 parameter bound, with no caller supplied bound.
- FR-015-AC-40 (examined): test compares generated source with the selected clause oracle source.
- FR-015-AC-42 (context_only): nonprecondition and unsupported shapes are refused; complete typed operator refusal remains planned.
- FR-015-AC-46 (examined): source test counts one cover and no assertion; ignored installed Kani control checks the satisfiable precondition.
- FR-015-AC-47 (examined): `#[trace("FR-015-AC-47", "TC-025")]` binds the identity test, whose equal, changed unwind and changed subject cases compare the record.
- TC-025 (examined): two focused generation/identity tests and one ignored installed Kani test were read; the reviewer did not run the heavy gate.
- Plan completion: not assessed. Optional semantic repository-wide review: not requested. The verdict covers the changed slice, not a whole-repository clean matrix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

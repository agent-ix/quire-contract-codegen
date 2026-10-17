---
id: "SR-12182"
title: "CG PR 421 spec review (evidence): AD-004 step 4b HarnessSpec and render"
type: SpecReview
analysis: evidence
review_set: subset
scope: "agent-ix/quire-contract-codegen@77f5178102658bdac3d77b4024d262f029e5da46 (PR #421, IR-858); spec/assurance/AD-004-cg-crate-layout.md (The HarnessSpec and render contract (step 4b), L-4, step 4b, import rule, Not verified), spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-112 to FR-015-AC-117), spec/kani/matrix/TC-025-bounded-kani-obligations.md (steps 42 to 47), spec/kani/matrix/tests.md; checked against src/ and tests/it/ at origin/main 0e0f92e"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: references
---

# SR-12182: CG PR 421 spec review (evidence): AD-004 step 4b HarnessSpec and render

## Summary

Ticket: IR-858. Evidence fit for FR-015-AC-112 to FR-015-AC-117 and P-3, checked against the kani-lane tests on
main (tests/it/kani_obligations.rs) and the scalar lowering refusals (scalar.rs).

Test is the right method for AC-112 to AC-115, and real Kani as a smoke (AC-117) is right for a byte-identical
port. A golden or differential comparison is the right primary evidence. The golden approach is non-circular in
principle, because the capture runs the unported code before the port; SR-12180 FND-002 covers its provenance
clause.

Units examined: FR-015-AC-112, FR-015-AC-113, FR-015-AC-114, FR-015-AC-115, FR-015-AC-116, FR-015-AC-117, AD-004-H-1, AD-004-H-2, AD-004-H-3, AD-004-H-4, AD-004-R-1, AD-004-P-1, AD-004-P-2, AD-004-P-3, AD-004-L-4, AD-004-import-rule, AD-004-step-4b, TC-025-step-42, TC-025-step-43, TC-025-step-44, TC-025-step-45, TC-025-step-46, TC-025-step-47, tests.md-FR-015-AC-112-117, tests.md-TC-025-row. Context only: FR-015-AC-7, FR-015-AC-58, AD-004-L-3.

## Verdict

Two medium gaps: the AC-117 controls do not exist in the form named, and the golden claim set does not exercise
the data flow the port changes. Fix both before the code PR relies on them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-117, P-3 item 3 and TC-025 step 47 call for 'the kani lane scalar tests' to run three controls, and two of them do not exist in that form. The existing scalar mutation control (tests/it/kani_obligations.rs:2950, Trace FR-015-AC-37) swaps the oracle's Add for Subtract; it is not `left + right + 1`, which is QSL-342 QI #10 and is routed through QSL (L-5). No scalar harness with an unsatisfiable input assumption can be built: H-3 item 4 refuses an inverted domain, and lower_scalar_claim/unsatisfiable refuse such bounds (tc_025_unsatisfiable_bounds_are_refused). Fix: name the existing mutation, and specify the L-4 control as a new hand-built spec with an unsatisfiable Assume, rendered by render and run under Kani. | spec/kani/functional/FR-015-bounded-kani-obligations.md:742, spec/assurance/AD-004-cg-crate-layout.md:569-574, spec/kani/matrix/TC-025-bounded-kani-obligations.md:436-438, tests/it/kani_obligations.rs:2950, tests/it/kani_obligations.rs:1685 |
| FND-002 | medium | The golden claim set varies only the operation and the operand ranges (four operations, a literal operand, an inferred operand, i64 extremes). The port changes another data flow: unwind, solver and options move from the request into the spec, and the identity reads them from the spec (H-1). With one fixed unwind and ceilings, a constructor that ignores the requested unwind or computes options from a default still matches the goldens. Fix: add claims at two unwind values (including MAX_OBLIGATION_UNWIND) and two ceilings values, and state that the golden test has no regenerate or bless mode. | spec/kani/functional/FR-015-bounded-kani-obligations.md:740, spec/assurance/AD-004-cg-crate-layout.md:561-565, spec/kani/matrix/TC-025-bounded-kani-obligations.md:426-428 |

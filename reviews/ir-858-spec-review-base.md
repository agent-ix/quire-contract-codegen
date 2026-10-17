---
id: "SR-12180"
title: "CG PR 421 spec review (base checklist): AD-004 step 4b HarnessSpec and render"
type: SpecReview
analysis: base
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

# SR-12180: CG PR 421 spec review (base checklist): AD-004 step 4b HarnessSpec and render

## Summary

Ticket: IR-858 (child of IR-348). PR: agent-ix/quire-contract-codegen#421 at 77f5178, base origin/main 0e0f92e.
Spec-only diff (+217/-11): AD-004 gains "The HarnessSpec and render contract (step 4b)" (H-1 to H-4, R-1,
P-1 to P-3), L-4 tightens to exactly one cover, FR-015 gains FR-015-AC-112 to FR-015-AC-117, TC-025 gains
steps 42 to 47, and tests.md gains an FR row and a TC-025 row.

- Ids: FR-015-AC-112 to AC-117 are new and unused on main; AC-105 to AC-108 belong to PR #417 and
  AC-109 to AC-111 to IR-598, so the gap is intentional. TC-025 steps 42 to 47 follow step 41.
- `quire validate --scope .` on the four changed files exits 0 (module-loader warnings only).
- Verification cells name a method only. No hash or pin is created; the AD-003 obligation digest
  compared by FR-015-AC-115 is the canonical identity digest the checklist allows.
- Two checklist hits: a source-text scan criterion (FND-001) and an evidence criterion bound to commit
  provenance (FND-002).

Units examined: FR-015-AC-112, FR-015-AC-113, FR-015-AC-114, FR-015-AC-115, FR-015-AC-116, FR-015-AC-117, AD-004-H-1, AD-004-H-2, AD-004-H-3, AD-004-H-4, AD-004-R-1, AD-004-P-1, AD-004-P-2, AD-004-P-3, AD-004-L-4, AD-004-import-rule, AD-004-step-4b, TC-025-step-42, TC-025-step-43, TC-025-step-44, TC-025-step-45, TC-025-step-46, TC-025-step-47, tests.md-FR-015-AC-112-117, tests.md-TC-025-row. Context only: FR-015-AC-7, FR-015-AC-58, AD-004-L-3.

## Verdict

The subsection decides the shape SR-785 FND-005 left open, keeps record.rs unchanged, and leaves 4c to 4h
undecided (one exception, SR-12183 FND-002). The FR row and TC rows trace correctly. Not mergeable on the
base checklist until FND-001 is fixed; FND-002 should be fixed with it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-015-AC-116 and TC-025 step 46 are source-text scans: no non-test string literal in scalar.rs spells an L-3 item, and no operation name, oracle symbol or the word scalar appears in a render.rs string literal. The checklist forbids a test case that scans source text, and existing use (L-3, FR-015-AC-58) is not a justification. The new clauses add scanned terms beyond the existing guard. Fix: drop the scan clauses; evidence R-1 item 3 behaviourally (FR-015-AC-113's hand-built specs carry non-scalar labels, ExprTexts and messages and the rendered source holds only what the spec supplied) and keep FR-015-AC-58's parse inspection of the emitted harness for the cover rule. | spec/kani/functional/FR-015-bounded-kani-obligations.md:741, spec/kani/matrix/TC-025-bounded-kani-obligations.md:433-435, spec/assurance/AD-004-cg-crate-layout.md:526 |
| FND-002 | medium | FR-015-AC-115 and P-3 items 1-2 make the equivalence evidence depend on commit history: golden files captured in the code PR's first commit, which the port commit does not edit, and a golden file edited in the port commit fails review. That is a recorded/reviewed-revision check, which the checklist disallows, and no test can fail it. Fix: state the observable only (the ported path renders the same artifact, identity, options and record as the pre-port renderer), and evidence it with a differential test that keeps today's render_scalar body as a cfg(test) reference and compares both over the claim set. That is non-circular without git history. If golden files stay, take the provenance clauses out of the AC. | spec/kani/functional/FR-015-bounded-kani-obligations.md:740, spec/assurance/AD-004-cg-crate-layout.md:561-567, spec/kani/matrix/TC-025-bounded-kani-obligations.md:426-428 |

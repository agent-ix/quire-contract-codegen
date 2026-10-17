---
id: "SR-12183"
title: "CG PR 421 spec review (scope boundary): AD-004 step 4b HarnessSpec and render"
type: SpecReview
analysis: scope-boundary
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

# SR-12183: CG PR 421 spec review (scope boundary): AD-004 step 4b HarnessSpec and render

## Summary

Ticket: IR-858. Boundaries: lowerer and renderer (ExprText), spec and identity (H-4), 4b and 4c to 4h, and
this PR against open PRs #417 and #406.

- 4c to 4h: ProofForContract, contract attributes, the frame's true cover, the collection domain, the corpus
  template and record.rs's fate are explicitly left open (AD-004:581-585). negotiate changes by one rename
  only. The one leak is FND-002.
- H-4: lowerers construct specs and negotiate does not. The public surface (KaniScalarObligationHarness,
  ScalarObligationIdentity, ObligationRecord, the negotiate_kani_obligations signatures) is unchanged. A
  HarnessSpecError maps to InvalidGeneratedSyntax, as harness_path's refusal does today.
- Overlap: PR #417 inserts FR-015-AC-105 to AC-108 at the same place, after FR-015-AC-104. That is a textual
  conflict with disjoint ids; whichever merges second rebases with 105-108 before 112-117. #417's tests.md
  edit is the AC-96..102 row, so no conflict there. #406's AD-004 hunk (:238, output/diagnostics.rs) is
  separate from this PR's :203 hunk. Neither is a spec defect.

Units examined: FR-015-AC-112, FR-015-AC-113, FR-015-AC-114, FR-015-AC-115, FR-015-AC-116, FR-015-AC-117, AD-004-H-1, AD-004-H-2, AD-004-H-3, AD-004-H-4, AD-004-R-1, AD-004-P-1, AD-004-P-2, AD-004-P-3, AD-004-L-4, AD-004-import-rule, AD-004-step-4b, TC-025-step-42, TC-025-step-43, TC-025-step-44, TC-025-step-45, TC-025-step-46, TC-025-step-47, tests.md-FR-015-AC-112-117, tests.md-TC-025-row. Context only: FR-015-AC-7, FR-015-AC-58, AD-004-L-3.

## Verdict

Scope holds with one medium boundary hole: ExprText admits statements, assertions and macros inside an
expression (FND-001). Fix FND-001; FND-002 and FND-003 are small.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | ExprText is the only lowerer-supplied text, but H-2 accepts any syn::Expr that does not spell kani:: or #[. That includes a block `{ stmt; assert!(..); x }`, a closure, `unsafe { .. }`, a loop, and any non-kani macro (assert!, panic!, unreachable!). A lowerer can then put statements and assertions inside an expression, including the cover's condition. 'They format no harness structure' (AD-004:418) and R-1 item 2's 'nothing a lowerer provides can change where the cover goes' are not enforced. Fix: refuse block, unsafe, closure, async and loop expressions and every macro outside an allowlist (scalar needs only matches!), and add these to FR-015-AC-112. | spec/assurance/AD-004-cg-crate-layout.md:470-480, spec/assurance/AD-004-cg-crate-layout.md:418, spec/assurance/AD-004-cg-crate-layout.md:519-521 |
| FND-002 | low | H-1 adds Statement::Assume 'because ... the contract families (4c) need them'; the scalar family uses none. That breaks the subsection's own rule, '4b adds no variant, field or helper for a family it does not port' (AD-004:435), and H-3 item 2 fixes Assume's position for 4c. Either leave Assume to 4c, or justify it as 4b's own: the L-4 control of SR-12182 FND-001 needs an unsatisfiable Assume, and the AD should say so. | spec/assurance/AD-004-cg-crate-layout.md:460-462, spec/assurance/AD-004-cg-crate-layout.md:431-436, spec/assurance/AD-004-cg-crate-layout.md:487 |
| FND-003 | low | P-1 and step 4b describe the rename render_scalar -> scalar_harness as 'one renamed call in negotiate.rs'. It also touches negotiate.rs's import (:85) and docs (:23), src/oracle/scalar/mod.rs:637 and :3158, tests/exact_scalar_support/agreement_cases.rs:98 and :102, and scalar.rs's own intra-doc links. List them, or keep the name: nothing in the contract needs the rename. | spec/assurance/AD-004-cg-crate-layout.md:539-541, spec/assurance/AD-004-cg-crate-layout.md:944-946, src/kani/generate/negotiate.rs:23, src/kani/generate/negotiate.rs:85 |

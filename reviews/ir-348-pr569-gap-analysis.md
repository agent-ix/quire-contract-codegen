---
id: SR-35026
title: "IR-348 CG569 gap-analysis (AD-004 L-8 state-frame slice)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-348; PR569; exact 34424f6115361c2bcf3b7afdacafc758b8012a00; base dd59aa14b8af9d8f34638669125736997817f419; src/core/ir/frame.rs, src/core/ir/mod.rs, src/kani/generate/frame.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
---

## Summary

Ticket: IR-348. This is a planless gap analysis of CG #569, scoped to its diff: a refactor that moves state-frame checked-node access into `core/ir/frame.rs`. The diff adds no requirement, acceptance criterion or trace tag, and removes none. The computed matrix (`quire matrix --scope . --format json`, quire 0.36.2) at the head reports 557 tagged, 301 untagged, 53 method-without-symbol and 16 tagged-by-ignored-test criteria, all of them repository-wide and unchanged by this diff. The binders into the changed files are FR-038-AC-1/3/6, FR-015-AC-26 and FR-014-AC-2, all on existing `kani/generate/frame.rs` tests. Those tests are unchanged and still present.

AD-004 status stays truthful. The PR does not edit AD-004. L-8 still reads "L-8 lands with the last typed-node-access PR". The PR body says the scalar, equality and function slices remain and that IR-348 stays open. No matrix row or spec text claims L-8 is done.

## Verdict

CONDITIONAL. There is no traceability, stub or coverage-inflation gap in the diff, and no unowned behavior: the moved code is owned by AD-004 L-8/step 3 and by the existing FR-015/FR-040 state-frame criteria. The one low finding is that the new L-8 boundary test binds to no criterion, so the computed matrix cannot see it. That is acceptable for an AD exit check, but it is recorded so nobody counts it as AC evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new test `core::ir::frame::tests::frame_generator_uses_typed_graph_and_operation_access` binds to no criterion: it has only a `/// Trace: AD-004 L-8.` doc line and no `#[trace]`. L-8 evidence is therefore invisible to the computed matrix. The test also scans only `kani/generate/frame.rs`, not the crate-wide L-8 grep gate that AD-004 names. That gate is correctly deferred to the last slice, but this test must not be cited as L-8 acceptance evidence. | src/core/ir/frame.rs:183-195 |

## Coverage

- Matrix: computed at the head; no criterion's status changes from this diff (no tags added or removed).
- Reverse gap: no new public behavior. `FrameScope`, `NodeShape`, `Located` and `Graph` are `pub(crate)` internals of an existing owned path.
- Stubs and inflation: none. The new helpers have real bodies, and the source-grep test asserts real absences that fail if a raw read is reintroduced into `kani/generate/frame.rs`.
- Focused runtime evidence (reviewer-run at the head): the state-frame and replay integration tests pass 114/114 (3 ignored), and the lib tests `core::ir::`/`kani::generate::frame::`/`replay::state_clause::` pass 7/7. `layout::l_2_no_file_but_the_crate_root_re_exports_an_item` fails on the new `Graph` re-export. That is a code defect, recorded as SR-35025 FND-001, not a traceability gap.
- Semantic review: skipped (not opted in; the change is a behavior-preserving move, traced manually in SR-35025).

Plan completion: not assessed

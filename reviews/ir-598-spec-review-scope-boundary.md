---
id: SR-12202
title: "Scope-boundary review of IR-598 per-family corpus retirement (CG #423 delta)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; PR #423 delta f22c59d9..1163a768 (IR-598): spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/spec.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/tests.md"
review_set: subset
date: 2026-10-17
---

## Summary

Ticket: IR-598. Checked the slice gates and boundaries (IR-858/4b, 4h, IR-268, IR-805 and its upstream owners, the no-wrapper rule) against the planner rulings on IR-598 and against AD-004 4b/4g/4h. Graph ownership (QSL-819, then IR-852 and IR-851, then IR-805) and the IR-854 division handoff are correctly outside CG. The collection slice's gate set is inconsistent with AC-109's own reasoning and with AC-95's end state, and its IR-268 gate cannot be evaluated from the spec.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-109 gates on 4b 'because the V2 path must already render through HarnessSpec and render.rs for the deletion to leave the family served'. AC-110 gates on 4b only, yet collection.rs renders its own template until step 4h (AD-004 :1043-1049). AC-95's end state requires every proof-eligible case to render through HarnessSpec, and also says AC-95 holds once AC-109..111 hold. AC-95 could therefore be claimed with 4h unlanded. Either gate AC-110 (or AC-95) on 4h, or narrow AC-95's HarnessSpec clause. | FR-015-AC-110, FR-015-AC-95, FR-015-AC-109, AD-004 (spec/kani/functional/FR-015-bounded-kani-obligations.md:801) |
| FND-002 | medium | The gate 'AC-96, -97 and -99 to -102 being backed (IR-268)' cannot be evaluated. IR-268 is Done, collection.rs and tagged tests exist (including the kani-lane tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies), yet FR-015:788-793 still mark those rows PLANNED, and tests.md says 'no symbolic collection or graph harness exists'. Name the concrete evidence that opens the gate, or update the stale status markers. | FR-015-AC-110, FR-015-AC-96, FR-015-AC-99, FR-015-AC-102 (spec/kani/functional/FR-015-bounded-kani-obligations.md:801) |
| FND-003 | low | The no-wrapper rule has no checkable form. A name search catches only the names it lists, so an adapter under a new name, for example one that builds a V2 request from `CheckedArithmeticRequest`, `CollectionQuery` or `GraphRequest`, passes. Add an inspection: each slice adds no interface-001 operation, and no src/ item converts a finite-input request type into a V2 generator call. | FR-015-AC-95, FR-015-AC-109, FR-015-AC-110 (spec/kani/functional/FR-015-bounded-kani-obligations.md:786) |

## Verdict

The boundaries are mostly right: CG owns no population input, division moves to IR-854, and every slice is gated on IR-858. Fix the 4h gap and give the IR-268 gate and the no-wrapper rule a checkable form.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at 455b1b1 (round 1) |
| FND-002 | fixed | Verified at 455b1b1 (round 1) |
| FND-003 | fixed | Verified at 455b1b1 (round 1) |

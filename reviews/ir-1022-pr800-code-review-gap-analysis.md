---
id: SR-17940486709564847047719
title: "IR-1022 PR #800 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #800 at 2a45f2e8 (base main cd660b50); IR-1022; FR-015-AC-143..148, TC-025 step 55, src/kani/generate/{clause/v2.rs,render.rs,render/constrained.rs,spec.rs}, src/oracle/{clause.rs,scalar/render.rs}, tests/it/{native_boolean_equality_differential.rs,kani_obligations_state_frame.rs}"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-1022. Planless gap analysis over the PR's new criteria, the computed matrix
(`quire matrix --scope .`), the tagged tests and the changed production code. Plan completion:
not assessed.

Matrix at 2a45f2e8: FR-015-AC-143 and AC-144 are tagged by
`native_clause_harness_forgets_fat_values_after_the_cover`. AC-145 is tagged by
`native_clause_oracle_stop_is_a_payload_free_copy_kind`. AC-146 is tagged by that test and by
`native_boolean_equality_and_inequality_match_the_typed_route`. **AC-147 and AC-148 are
untagged**, although both are Implemented with verification method `Test (TC-025)`.

Code ownership: the thin stop type and measured helper (AC-145), the Forget statement, validation
and rendering (AC-143/144), and the native Boolean equality body (AC-146) are all owned. The
per-shape native arms in `src/kani/generate/render/constrained.rs` (`completed_arms`, `stop_arms`,
the safety arm and `observation_lines`) are owned by no criterion, since AC-145 speaks only of
`oracle_clause`. They are exercised by the existing constrained tests (green), but nothing states
that a constrained harness's panic semantics are unchanged.

Bindings: three trace tags are correct as to subject. The AC-146 binding to the differential test
is real but weak in intent (see code-review SR-17940486709564847047718 FND-001).

## Verdict

Gaps: two Implemented rows with no binder (FND-001), and one production change with no owning
criterion (FND-002). Neither changes the code verdict. FND-001 has to be resolved with the
spec-review findings before the rows read Implemented.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-147 and FR-015-AC-148 read Implemented with verification method `Test (TC-025)`, but `quire matrix` shows them untagged (no binder). The evidence is a one-off manual `cargo kani` run outside the repo. Either add a kani-lane test tagged to them (ignored by default like the other real-Kani tests) or change the method to the evidence actually held (Demonstration/Analysis with a durable record) | spec/kani/functional/FR-015-bounded-kani-obligations.md:962-963 |
| FND-002 | low | The constrained renderer's native per-shape arms (`completed_arms`, `stop_arms`, the native safety arm and the added `observation_lines` prefix `Ok(value) => assert!(value,`) are a behaviour change to generated constrained harnesses that no criterion owns. State in AC-145 (or a sibling) that constrained harnesses keep the same panic per stop kind with the native oracle | src/kani/generate/render/constrained.rs:199-216, src/kani/generate/render/constrained.rs:314-327, src/kani/generate/render/constrained.rs:470 |

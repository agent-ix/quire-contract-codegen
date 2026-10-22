---
id: SR-15412
title: "IR-948 PR 519 V2 invariant acceptance gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@c9b9fd3e2d4e29e9701cf9c1968c857418fe4cf7; spec/kani/functional/FR-015-bounded-kani-obligations.md; spec/kani/matrix/TC-025-bounded-kani-obligations.md; src/kani/generate/clause/v2.rs; src/kani/generate/negotiate.rs; tests/it/kani_obligations_state_frame.rs; tests/state_frame_support/native_twin.rs; FR-015-AC-38,39,40,46,49; TC-025"
review_set: subset
---

## Summary

Ticket: IR-948. Diff-scoped acceptance chain for CG PR 519 at exact head c9b9fd3e2d4e29e9701cf9c1968c857418fe4cf7. `quire matrix --scope . --format json` found zero test binders for FR-015-AC-38, 39, 40, 46, and 49. The new tests trace only TC-025. The PR body also claims refusal of an unbounded invariant field, but its added tests cover only bounded one-field and two-field packages.

## Verdict

**CONDITIONAL** — bind the new invariant tests to the criteria they assert and add a focused unbounded selected-model-field refusal test for the claimed FR-015-AC-39 boundary. The examined bounded path has an actual generated oracle and non-vacuous assertion; no stub was found. The full four-kind requirement remains in later slices.

## Coverage

- FR-015-AC-38 (examined): generated invariant harness kind and separate symbol checked by the new default test.
- FR-015-AC-39 (examined): one- and two-field inclusive bounded inputs checked; the unbounded invariant refusal has no new test.
- FR-015-AC-40 (examined): generated artifact contains the selected clause oracle source.
- FR-015-AC-46 (examined): one cover follows the assertion and assumptions; real proof remains ignored until the Kani lane.
- FR-015-AC-49 (examined): default test checks a subject-free assertion and the ignored real Kani case expects healthy verified and mutant falsified outcomes.
- TC-025 (examined): the matrix document describes the V2 invariant oracle and backend case.
- Plan completion: not assessed. Optional repository-wide semantic review: not requested. This verdict is scoped to the PR diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new invariant tests are tagged `TC-025` only. The computed matrix has zero binders for FR-015-AC-49 and the other invariant criteria, so the acceptance chain is incomplete. | tests/it/kani_obligations_state_frame.rs:447 |
| FND-002 | medium | No new test passes an invariant reading an unbounded selected model field to negotiation and asserts a typed refusal with no harness, despite the PR's refusal claim and FR-015-AC-39 boundary. | tests/it/kani_obligations_state_frame.rs:503 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Independent `quire matrix --scope . --format json` at exact head bee0f85c lists binders for FR-015-AC-38, 39, 40, 46, and 49. |
| FND-002 | fixed | The new QSL twin removes the selected model balance range, and `qsl_invariant_unbounded_field_refuses_without_harness` checks the exact unsupported node identity and empty V2 harness collection. |

## Disposition verdict

**PASS** for the invariant slice at exact head bee0f85c4c2de810a7571a13f50e6b0bab344b49. The real healthy and mutant Kani case remains for the author’s final-head Kani gate.

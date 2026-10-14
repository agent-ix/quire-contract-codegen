---
id: SR-8671
title: "IR-461 PR 376 gap analysis: state-frame role parity criteria"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; src/kani/generate/role_parity_tests.rs, tests/it/kani_obligations_state_frame.rs, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md; FR-015-AC-60, FR-015-AC-68"
review_set: subset
---

## Summary

The computed matrix tags FR-015-AC-60 and FR-015-AC-68 through the new tests, and both focused tests pass. The AC-68 tag covers only two of its three required cases, and the authored coverage status still describes these criteria as planned and untraced.

## Verdict

**CONDITIONAL** — the tagged AC-68 test omits the refused-lowering case; the coverage descriptions also need to reflect the new evidence.

## Coverage

- FR-015-AC-60, examined: direct role equality for both supported roles, record order, symbols, and pair equality with the public entry.
- FR-015-AC-68, examined: frame-only refusal, contract-only refusal, and the separate both-role lowering refusal requirement.
- TC-025 steps 20 and 28, examined: role parity and independence steps; the old planned-status explanation is stale.
- `quire matrix --scope . --format tsv` reports both ACs tagged by the new test file. Existing `tests/it/kani_obligations_state_frame.rs` checks the both-role lowering refusal but explicitly omits the AC-68 tag.
- Reverse gap: the new tests are owned by the two stated criteria. No production behavior changed. No stubs or copied files were added. Plan completion: not assessed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The AC-68 trace claims coverage without checking the refused-lowering case | src/kani/generate/role_parity_tests.rs:124 | correct-requirement-no-evidence |
| FND-002 | low | AC-60 and AC-68 still say planned and untraced after their tests gained trace tags | spec/kani/functional/FR-015-bounded-kani-obligations.md:682 | wrong-requirement |

The new `state_frame_one_role_refused_matches_direct_generation` test calls `compare_roles` only with an all-fields frame and a negated condition. FR-015-AC-68 additionally requires a lowering refusal to record both roles alike. The existing integration test asserts that case at `tests/it/kani_obligations_state_frame.rs:2188`, but its doc comment expressly says it does not back AC-68 and gives only `Trace: TC-025`. Thus the computed matrix's AC-68 tagged status overstates what the tagged test establishes.

The AC-60 and AC-68 rows still contain `PLANNED (IR-461)` explanations saying the crate-internal comparison cannot be tested and that no test carries a trace tag. TC-025's steps 20 and 28 make the same claim. The added in-crate tests invalidate that status and explanation; readers and automated trace checks now receive conflicting coverage claims.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 6d453c2, the AC-68 tagged test exercises a QSL-emitted refused lowering, checks equal direct `NotLowered` refusals for both roles, both negotiated `RequiresBound` records with the same node, no harness, and the two-role entry's refusal. |
| FND-002 | fixed | At 6d453c2, the AC-60 and AC-68 rows say `COVERED`, TC-025 steps 20 and 28 describe the crate-internal comparisons, and the hand-written matrix row and integration-test comments no longer claim these criteria are planned or untraced. |

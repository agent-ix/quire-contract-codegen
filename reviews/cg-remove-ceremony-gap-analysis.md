---
id: SR-043
title: "Gap analysis — quire-contract-codegen PR #189 remove ceremony"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@759c5d20ab3a4472b3916a59e5245cef898d4c56; spec/test-matrix.md, spec/functional/**, spec/nonfunctional/**, tests/, src/"
review_set: subset
---

## Summary

This compares the requirements and tests at the PR head against `origin/main` 6e0c518, using `quire coverage --scope . --strict` on both. Both exit 1, main before and head after. Main backs 232 of 297 rows and head backs 209 of 276. The rows that disappeared belong to requirements the PR deleted, which is expected. But four acceptance criteria that are still present lost their only backing test. Several more are still marked `✅ Covered` in the matrix, although the code they describe was removed. No ticket id is derivable.

## Verdict

**FAIL**. The PR deletes code and tests for pins, attestations, the tool probe and the goldens, but leaves the acceptance criteria that require them in the spec. The spec and matrix therefore claim behaviour the code no longer has. The fix is to delete or rewrite those criteria in this PR; adding tests back is not the remedy. `status_lies` is 0 on both main and head, so quire does not catch this; I found it by reading the code.

These rows went away cleanly because the requirements were deleted: FR-023 (4 rows), the old FR-006 (10, replaced by the new FR-006 with 3), interface-001 AC-3 and AC-5, and the matching test-matrix rows.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high | FR-017-AC-1 and AC-3 (refuse on pin drift in any of the six fields) lost their backing tests, because pins were deleted. The ACs remain, and the TC-027 matrix row still lists them. | spec/functional/complete-v1/FR-017-pinned-kani-execution-evidence.md:135-137; spec/test-matrix.md:332 |
| FND-002 | high | FR-017-AC-6 requires evidence to carry the schema, identity and source digests, the measured pins, the lockfile, oracle digest and runtime revision. `KaniExecutionEvidence` no longer has these fields. AC-11 requires identity-pin checks and drift refusal. The matrix marks AC-11 `✅ Covered`, and `tests/it/kani_obligations.rs:1800` still traces AC-6. | spec/functional/complete-v1/FR-017-pinned-kani-execution-evidence.md:140-145; src/kani_execution.rs:353; spec/test-matrix.md:68 |
| FND-003 | high | NFR-002-AC-1 requires a sealed attestation for every artifact. It lost its backing test, but the TC-001 matrix row still counts it as `✅ Covered`. NFR-002-AC-3 lost its only tagged test when `shared_assurance.rs` was deleted, although the conformance example still exercises it. | spec/nonfunctional/NFR-002-provenance-boundary.md:42-44; spec/test-matrix.md:290; spec/test-matrix.md:314 |
| FND-004 | high | FR-019-AC-6 and TC-030 still require tool-probe records (`tool-unavailable`, and `failed` after a passing probe). `record_tool_probe` and both of its tests were deleted. The matrix marks it `✅ Covered` and still names `record_tool_probe`. | spec/functional/complete-v1/FR-019-capability-settlement.md:100-126; spec/test-matrix.md:72; spec/test-matrix.md:248-252 |
| FND-005 | high | FR-015-AC-2 requires the harness identity to record launcher and driver digests, Kani and CBMC versions, the adapter profile, the oracle crate digest and the runtime revision. `KaniObligationIdentity` has none of these, yet tests still trace FR-015-AC-2. | spec/functional/complete-v1/FR-015-bounded-kani-obligations.md:31-44; src/kani_obligations.rs:463-490 |
| FND-006 | high | FR-003-AC-4 requires the proof graph to keep cargo-kani 0.67.0, the executable digest and the adapter profile. The PR deleted exactly those fields from `kani-proof-graph-v2.schema.json`. The matrix marks AC-4 `✅ Covered`. | spec/functional/FR-003-kani-lowering.md:79; spec/test-matrix.md:26 |
| FND-007 | high | FR-001-AC-3 and AC-7 require attestations. FR-013-AC-3 and AC-4 require sealing through the real CLI and attestation argv. Both are marked `✅ Covered`, and `bound_strategy_generation.rs:1436` traces FR-013-AC-3 and AC-4 with a test that does neither. | spec/functional/FR-001-deterministic-oracles.md:94-98; spec/functional/strategies/FR-013-it010-consumable-output.md:88-89; tests/it/bound_strategy_generation.rs:1436 |
| FND-008 | high | FR-022 and TC-033 still describe `KaniToolPins`, `AttestationContext` and an `UnpinnedBackend` refusal, none of which exists any more. The matrix marks FR-022 `✅ Covered`. | spec/functional/complete-v1/FR-022-routed-generation.md:60; spec/functional/complete-v1/FR-022-routed-generation.md:226; spec/test/complete-v1/TC-033-routed-generation.md:115 |
| FND-009 | high | FR-014-AC-4 ("match the committed golden output") and FR-018-AC-10 ("committed golden crate… re-blessed golden") still require goldens that were all deleted. Both are marked `✅ Covered`. TC-024 and TC-029 were updated, but these FRs were not. | spec/functional/complete-v1/FR-014-exact-scalar-oracles.md:286; spec/functional/complete-v1/FR-018-composite-equality-oracles.md:203 |
| FND-010 | medium | FR-016 has no remaining test that shows the generated obligation `ensures` is load-bearing: the spine mutation control was removed (SR-042 FND-003). FR-015's routed scalar lane no longer verifies with real Kani (SR-042 FND-001). | tests/it/skeleton_spine.rs:374; tests/it/kani_obligations.rs:2057 |

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

## Dispositions

Round 1, reviewed at c0cc093bb657a92d280159e16424518e5440fd83. `quire coverage --strict`: head 202/274, main 232/297; both exit 1. Where the spec file is outside this PR, the matrix row is corrected here and the text fix is deferred to #186.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | matrix fixed in d82ac87 (FR-017-AC-1/3 are Planned at spec/test-matrix.md:71); the FR-017 text is #186 |
| FND-002 | deferred | matrix fixed in d82ac87 (AC-6/11 Planned); the FR-017 text is #186 |
| FND-003 | fixed | d82ac87: NFR-002-AC-1 now reads "header names the generator and the requirement, revision and clause"; NFR-002 rows are Planned |
| FND-004 | deferred | matrix fixed in d82ac87 (FR-019-AC-5/6 Planned, no `record_tool_probe`); the FR-019 text is #186 |
| FND-005 | deferred | matrix fixed in d82ac87 (FR-015-AC-2 Planned); the FR-015 text is #186 |
| FND-006 | fixed | d82ac87: FR-003-AC-4 now requires the option vector, subject ABI and domain bounds, with no version or digest |
| FND-007 | fixed | d82ac87: FR-001-AC-3/7 and FR-013-AC-4 rewritten, FR-013-AC-3 deleted. See FND-012 for the dangling tag |
| FND-008 | deferred | TC-033 fixed in d82ac87; FR-022 is #186 |
| FND-009 | deferred | matrix fixed in d82ac87 (FR-014-AC-4 and FR-018-AC-10 Partial); the FR text is #186 |
| FND-010 | fixed | 87963e9: spine mutation control restored; routed scalar Kani passes |

## New findings (disposition pass 1)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-011 | high | The matrix still marks FR-017-AC-8 and AC-9 `✅ Covered`, but their only test (the source census) was deleted in 87963e9 and nothing traces them now. Its prose at :142-144 still names that census. Mark them Planned or delete the ACs. | spec/test-matrix.md:69; spec/test-matrix.md:142-144 |
| FND-012 | high | TC-028 still names `tests/it/interface_001.rs` as its implementation, but the file was deleted, and the matrix marks interface-001-AC-4 and TC-028 `✅ Covered`. `GenerationTerminalState::ALL` is now tested by nothing. Either delete TC-028 and those ACs, or back AC-4 with a plain unit test. | spec/test/complete-v1/TC-028-interface-001-declared-surface.md:33; spec/test-matrix.md:224-225; spec/test-matrix.md:259; spec/test-matrix.md:311; spec/interface/interface-001-codegen-api.md:363-373; spec/index.md:82 |
| FND-013 | medium | A test still traces FR-013-AC-3, which was deleted. | tests/it/bound_strategy_generation.rs:1436 |

### Round 2

Reviewed at 476dbd1799cdd223b8a09fd79a11fd80e56dd837. `quire coverage --strict` exits 1 (198/265 rows backed); `make spec` exits 0.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-011 | fixed | cba94dd: FR-017-AC-8 and AC-9 are removed from the matrix. The FR-017 AC text is deferred to #186. |
| FND-012 | fixed | cba94dd and 476dbd1: TC-028 is deleted. interface-001-AC-1, AC-2 and AC-4 are removed along with their matrix rows and open items. They could only be verified by a public-API census or document-parsing test, which the owner ruled out. |
| FND-013 | fixed | cba94dd: the FR-013-AC-3 trace is removed (tests/it/bound_strategy_generation.rs:1436). |

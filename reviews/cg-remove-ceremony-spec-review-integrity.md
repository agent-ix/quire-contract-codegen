---
id: SR-044
title: "Spec integrity review — quire-contract-codegen PR #189 remove ceremony"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@759c5d20ab3a4472b3916a59e5245cef898d4c56; spec/**, README.md, CLAUDE.md, Makefile"
review_set: subset
---

## Summary

This reviews the PR's spec edits: deleting FR-023, TC-034, the old FR-006, TC-008 to TC-010 and TC-012 to TC-013; adding the new FR-006-generation-conformance; and editing FR-016, StR-001, interface-001, TC-024, TC-027 to TC-029, TC-031, TC-032, the matrix and the index. It also records spec text still in the tree that states pins, digests or attestations. `make spec` fails at both main and head with the same two documents, AP-001 and MP-001, so this PR adds no validation failure. Nothing under `reviews/`, `plan/` or `planning/` was touched except the expected deletion of `planning/draft-dependency-pins.md`.

## Verdict

**FAIL**. The PR's own edits are consistent. The matrix and TC files cross-check: every TC in the matrix exists, every cited AC is defined, and no FR-023, TC-034 or TC-008 to TC-013 ids remain outside history. FR-016 matches the code. But StR-001-VC-4, which the PR edited, still names "committed backend pins", and specs across the tree still state tracking requirements that the P0 rule says to delete. SR-043 covers the ACs whose code is gone. This document lists the prose and the assurance documents.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high | StR-001-VC-4 was edited by this PR and still says "generated under the committed backend pins". No pins exist. | spec/stakeholder/StR-001-traceable-generation.md:72 |
| FND-002 | high | FR-017 and TC-027 are still titled "pinned Kani execution evidence". FR-017's body (21-23, 37, 56, 60-70, 98-110) and TC-027's Pins procedure (34-40, 63-69, 87-118) describe pins and lockfile digests that were removed (case 6). | spec/functional/complete-v1/FR-017-pinned-kani-execution-evidence.md:3; spec/test/complete-v1/TC-027-pinned-kani-execution-evidence.md:3 |
| FND-003 | high | NFR-002 still states attestation sealing and has a "Qualification Integrity" narrative citing `assurance-chain`, `upstream-identity`, `tests/shared_assurance.rs`, TC-011 and `quoin seal-attestation`. All of these were deleted (cases 4, 6 and 7). | spec/nonfunctional/NFR-002-provenance-boundary.md:17-98 |
| FND-004 | high | The assurance documents still describe the deleted chain. MP-001 lists `scripts/assurance_chain.py`, `assurance/change-assurance.json`, `build.rs`, attestation sealing and TC-012. AD-001 references FR-023, the claimed-module gate and committed pins. AA-001 and CAC-001 cover goldens, attestations and `version_pins`. AP-001 says "pinned … backend versions" (case 4). | spec/assurance/MP-001-codegen-measurements.md:27; spec/assurance/AD-001-codegen-architecture.md:49; spec/assurance/AD-001-codegen-architecture.md:116; spec/assurance/AA-001; spec/assurance/CAC-001 |
| FND-005 | high | ADR-002 names `KaniToolPins::pinned` and `quire.codegen.kani-execution/v1`. ADR-003 names the claimed-module gate FR-023. | spec/decisions/ADR-002-backend-adapter-boundary.md:38; spec/decisions/ADR-003-kani-tractability.md:111 |
| FND-006 | high | FR-008-CON-1 asserts pinned revisions `04eb6f8` and `690bde7` (case 3). FR-001 (lines 58-61) requires source revision, dirty state and a lowering-implementation digest (case 7). FR-003 (lines 63-66) requires pinning cargo-kani 0.67.0 and its executable digest (case 3). | spec/functional/strategies/FR-008-bound-domain-strategy-admission.md:119; spec/functional/FR-001-deterministic-oracles.md:58-61; spec/functional/FR-003-kani-lowering.md:63-66 |
| FND-007 | high | Version assertions (case 3): interface-001 `coverage_analysis_slice` names "cargo-llvm-cov 0.9.0 with rustc 1.94.1" (this predates the PR). `spec/evidence/suites.md` has a tool-version column and "rustup run 1.98.1". NFR-001 names "golden-approval-testing" (case 5). | spec/interface/interface-001-codegen-api.md; spec/evidence/suites.md; spec/nonfunctional/NFR-001-reproducible-atomic.md:34 |
| FND-011 | medium | Stray attestation wording in FR-002:55, FR-005:40 and :49, TC-003:21 and :31, TC-006:28, and TC-007:21 ("proof attestations with golden"). | spec/functional/FR-002-tristate-proptest.md:55; spec/functional/FR-005-cli-conformance.md:40; spec/test/TC-007 |
| FND-008 | low | The spec index is stale. Line 60 says "Reviewers inspect proof attestations" and line 79 says "FR-017 pinned execution". The skipped-numbers note at line 86 omits FR-023, TC-008 to TC-010, TC-012, TC-013 and TC-034. Line 96, "FR-014 to FR-025 … TC-033 to TC-036", spans ids that no longer exist. | spec/index.md:60; spec/index.md:79; spec/index.md:86; spec/index.md:96 |
| FND-009 | low | The new FR-006 specifies the exit-code contract of the conformance example, which is test tooling rather than a product capability. It is the old FR-006-AC-8 to AC-11 without the census, kept so TC-032 has something to trace to. It declares `implements interface-001`, but interface-001 has no conformance operation. The owner should decide whether a requirement for test tooling is wanted. | spec/functional/FR-006-generation-conformance.md |
| FND-010 | low | CLAUDE.md says "rust-toolchain.toml pins to stable", but it pins 1.98.1. | CLAUDE.md:32 |

## Dispositions

Round 1, reviewed at c0cc093bb657a92d280159e16424518e5440fd83. `make spec` exits 0 at head; main exits 2 on AP-001/MP-001, which this PR fixed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d82ac87: StR-001-VC-4 now reads "generated for its routed backend" |
| FND-002 | deferred | TC-027 retitled and its pins procedure removed in d82ac87; FR-017's title and body are #186 |
| FND-003 | fixed | d82ac87: NFR-002 qualification narrative removed |
| FND-004 | deferred | MP-001, AA-001, AP-001 and CAC-001 cleaned in d82ac87; AD-001 is #186. See FND-012 |
| FND-005 | deferred | ADR-001, ADR-002 and ADR-003 are #186 |
| FND-006 | fixed | d82ac87: FR-008-CON-1, FR-001 and FR-003 have no pins |
| FND-007 | fixed | d82ac87: no cargo-llvm-cov versions, no tool-version column, no goldens in NFR-001 |
| FND-008 | fixed | d82ac87: index.md lines 60, 79, 86 and 96 updated |
| FND-009 | still-open | FR-006 still declares `implements interface-001`, and interface-001 has no conformance operation. Waiting on an owner decision |
| FND-010 | fixed | d82ac87: CLAUDE.md now says the toolchain file selects 1.98.1 |
| FND-011 | fixed | d82ac87: no attestation wording left in FR-002, FR-005, TC-003, TC-006 or TC-007 |

## New findings (disposition pass 1)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-012 | low | Stale lines in assurance documents this PR touched. MP-001:63 still prints `codegen.generation-conformance/v1`, although `PROTOCOL` is gone. MP-001:87-88 says `make msrv` is read from `build-finished`, which was chain behaviour. CAC-001:13-14 keeps `version_pins: rust-msrv`. | spec/assurance/MP-001-codegen-measurements.md:63; spec/assurance/MP-001-codegen-measurements.md:87-88; spec/assurance/CAC-001-codegen-contract.md:13-14 |

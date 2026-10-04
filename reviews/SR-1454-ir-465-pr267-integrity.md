---
id: "SR-1454"
title: "CG PR 267 spec review (integrity): repeated dependency identity placement and timing"
type: SpecReview
analysis: integrity
review_set: subset
date: "2026-10-04"
scope: "agent-ix/quire-contract-codegen@f67151a2ee745368bce77d9de2538e6ea88a4ef5; spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md, spec/kani/matrix/tests.md, spec/assurance/AD-003-evidence-chain.md, spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
---

# SR-1454: CG PR 267 spec review (integrity)

## Summary

Ticket: IR-465. Checked consistency across FR-029, FR-030, TC-040, TC-041, `tests.md`, AD-003,
AD-004 and interface-001 for the repeated-identity rule, and whether the buildable-now half has a
criterion and a test it can bind to. The partition of CG-raised failures is complete, the AC ids
are unchanged, and the matrix rows match the Status sections.

## Verdict

Two findings, one medium and one low. Neither blocks on its own, but the medium one should be
fixed before the code PR, because otherwise the code PR has no criterion to trace its changed
test to.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The buildable-now half of the rule has no criterion that can be built now, and it sits in the wrong requirement. The two new Behavior bullets ("The replay package builder shall define no duplicate-identity error ..." and "shall refuse a repeated dependency identity only through QSL's `DependencyInput::new` ...") constrain FR-016's replay package builder (`ReplayInputs::admit`), but they are written into FR-029, the Kani run-outcome map. Their only criterion, FR-029-AC-14, bundles the builder refusal with the `ReplayRefused` mapping. FR-029 Status, `tests.md` and TC-040 Status all list AC-14 as waiting on QSL's `Inconclusive`. The existing test `tc_026_a_lock_repeating_a_dependency_is_refused` (`tests/it/skeleton_spine.rs`, tagged TC-026) expects `Duplicate` and will change in the code PR. FR-029 Status mentions it only as "the existing test that expects the repeated identity as `Duplicate`", without naming it. FR-016 has no AC for a repeated identity (AC-15 to AC-19 cover ordering, unsupplied, mismatch, unselected and shared owner), so the changed test would trace to TC-026 with no owning criterion. Fix: state the builder refusal as an FR-016 AC traced to TC-026, for example "a lock selecting one library twice is refused as `DependencyLockError::Input(DuplicateIdentity)`, `invalid_package`/`conflicting-definition`". Leave FR-029-AC-14 as the mapping only, and name the test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:137-142,169,218-223; spec/replay/functional/FR-016-witness-native-replay.md:135-139; spec/kani/matrix/tests.md:36 |
| FND-002 | low | AD-003 R-Q1 and FR-029 Status say the repeated identity "settles by timing: `Declined` before a backend run, `Inconclusive(ReplayRefused(invalid_package))` after a refutation". In CG it never settles `Declined`. `ReplayInputs::admit` runs only inside `ReplayPackage::new` and the frame replay, both after Kani refuted. R-Q1 (a) and the FR-030 Behavior bullet ("`Declined` only from `Refused`, `InvalidInput` and `IncompleteInput`") already say so. The restated general QSL rule invites a coder or tester to look for a `Declined` case that FR-030 forbids. Fix: say that in CG the repeated identity arises only after a refutation, so it is always `ReplayRefused`. The `Declined` half is QSL's general rule, with no CG producer. | spec/assurance/AD-003-evidence-chain.md:377; spec/kani/functional/FR-029-run-outcome-terminal-record.md:211-216 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The kani `tests.md` FR-030 row still says "and AC-2's code half on QSL's pending `Declined` code (IR-465)". The fix round made FR-030-AC-2 cause-only, and FR-030 Status now says "AC-2 and the other rows can" be built. AC-2 has no code half any more, so the matrix row contradicts the Status and the criterion. This PR added the row text in its first commit. Fix: drop the clause, or replace it with a pointer to the Status note. | spec/kani/matrix/tests.md:37 |

## Dispositions

Round 1, reviewed at 69708045e4fcebbc491d3d6c8d9a3c391bc5bd30 (fix commit 6970804 on top of f67151a).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 69708045e4fcebbc491d3d6c8d9a3c391bc5bd30: new FR-016-AC-24 (the id was never issued on `origin/main`; FR-016 ended at AC-23) states the builder refusal for a lock whose only defect is the repeat: `DuplicateIdentity`, `invalid_package`/`conflicting-definition`, reaching the caller as `DependencyLockError::Input`, Test (TC-026). TC-026 adds the procedure sentence, an expected result, and a Status that names `tc_026_a_lock_repeating_a_dependency_is_refused` as the test that changes. The replay `tests.md` has a planned FR-016-AC-24 row, and the TC-026 summary lists AC-24. The FR-029 builder bullets are gone, and FR-029-AC-14 is now the mapping only, citing FR-016-AC-24. |
| FND-002 | fixed | 69708045e4fcebbc491d3d6c8d9a3c391bc5bd30: AD-003 R-Q1 says the settlement is by timing "in QSL's terms ... CG builds the lock only after a refutation, so for CG it is always `ReplayRefused`". FR-029 Status says the same. |

Round 2, reviewed at 1f6d3c57b9d969379a2ba1fe4a2b93304c47f69a (fix commit 1f6d3c5 on top of 6970804).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 1f6d3c57b9d969379a2ba1fe4a2b93304c47f69a: the kani `tests.md` FR-030 row now reads "AC-9, AC-12 and AC-13 wait on QSL's unmerged `Inconclusive` terminal value (IR-465); AC-2 asserts the cause only and is buildable now". That agrees with FR-030 Status and with merged QSL `Declined(ProofRefusalCause)`. |

---
id: "SR-626"
title: "IR-92 slice 1 gap analysis: FR-016 acceptance criteria to tests for the replay package and verdict"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@9a299b1f4e7c9f22c2ac571e56fa5d68cced70f0; spec/functional/complete-v1/FR-016-witness-native-replay.md, spec/test-matrix.md, spec/interface/interface-001-codegen-api.md, src/spine_replay.rs, src/kani_witness_join.rs, tests/it/skeleton_spine.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
---

# SR-626: IR-92 slice 1 gap analysis

## Summary

Ticket: IR-92 (slice 1), also IR-290. PR: agent-ix/quire-contract-codegen#201, head 9a299b1.
This is a manual check that maps acceptance criteria to tests, scoped to the PR diff. Plan
completion: not assessed.

## Method

I mapped each FR-016 acceptance criterion the new tests tag (AC-1, AC-2, AC-3, AC-4, AC-9) to the
test that carries the tag. I also checked AC-13, because the new code implements it. For each
test, I confirmed that the assertion fails when the code is wrong, using the source mutations
recorded in SR-625. I compared the tags with the rows of `spec/test-matrix.md`. I checked whether
any requirement owns the new behaviour of filling `package.dependencies`.

What I found for each criterion:
- **FR-016-AC-1** is tagged on `tc_026_an_undecodable_counterexample_is_evidence_failure`. The test
  is real (M7 killed), but it covers one garbage string.
- **FR-016-AC-2** is tagged on `tc_026_an_out_of_domain_counterexample_is_evidence_failure`. The
  binding is correct (M1 killed), but no test sits at a boundary (M2 survives).
- **FR-016-AC-3** is tagged on `tc_026_an_in_domain_counterexample_the_twin_falsifies_is_reproduced`.
  The binding is correct.
- **FR-016-AC-4** is tagged on `tc_026_a_counterexample_the_twin_holds_is_evidence_failure`. The
  binding is correct (M8 killed).
- **FR-016-AC-9** is tagged on `tc_026_the_request_package_reference_carries_the_lock_dependencies`.
  The binding is wrong: that test never replays.
- **FR-016-AC-13** is implemented at src/spine_replay.rs:410-418, but no test covers it (M3
  survives).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The dependency test is tagged FR-016-AC-9, the reproduced-with-evaluated-witness settlement. It never calls `replay` and asserts only the wire shape. No FR-016 criterion owns filling `package.dependencies` (IR-290), so the new behaviour has no owning requirement. Imported-dependency replay is untested: QSL refuses the request that test builds as `Unselected`. | tests/it/skeleton_spine.rs:299-334, src/spine_replay.rs:285-316 |
| FND-002 | medium | FR-016-AC-13 (a reproduced settlement with a category other than violation yields mismatch, never a reproduced failure) is implemented in the new verdict mapping, but no test covers it. Mutant M3 survives. | src/spine_replay.rs:410-418 |
| FND-003 | low | The test matrix is not updated. Row 52 still says FR-016-AC-1 "carries no trace tag", but it now does. Row 53 lists AC-2, AC-3 and AC-4 as Planned, but they are now tagged. | spec/test-matrix.md:51-53 |
| FND-004 | low | The FR-016-AC-1 tag overclaims. The criterion reads "every retained witness decodes or is refused as malformed"; the test covers one non-playback string. Matrix row 52's own objection still stands: the refusal is a wrapped `KaniOutcome`, not a malformed-witness result. | tests/it/skeleton_spine.rs:394-411, spec/test-matrix.md:52 |

## Verdict

The tags on AC-2, AC-3 and AC-4 are backed by oracles that fail when the code is wrong. AC-1 is
real but narrower than its tag. The AC-9 tag on the dependency test is a wrong binding. AC-13 is
implemented but untested. Filling the dependencies (IR-290) is traced to no requirement and
reaches no replay QSL admits. Not mergeable until FND-001 and FND-002 are fixed or given a
disposition.

## Dispositions

Round 1 was reviewed at f4fff61ad01aed9857a210d0b31373c59120e727. The matrix now marks FR-016-AC-1 through AC-5, AC-8 through AC-11 and AC-13 Covered. Each of those rows has a real test, and I confirmed each test fails when the code is wrong: M1, M2, M3 and M13 are killed. AC-6, AC-7 and AC-12 stay Planned.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 36a9062: the dependency test carries only a TC-026 tag. A new test asserts QSL's `Unselected` refusal, and TC-026 (3e09e6b) states that filling is implemented while imported-dependency replay is unreachable until `call_site` accepts a dependency input. |
| FND-002 | fixed | 36a9062: the `only_a_reproduced_violation_reproduces` unit test is tagged FR-016-AC-13. Mutant M3 is killed. |
| FND-003 | fixed | 3e09e6b: spec/test-matrix.md rows 51-52 are rewritten to match the tags. |
| FND-004 | fixed | 36a9062: the AC-1 test covers no playback block, an arity mismatch and a width mismatch, each asserting its decoder cause code. A separate FR-016-AC-5 test covers a transcript for another harness. The refusal is now FR-016's decode-cause evidence failure (FR-016 Outputs reworded in 3e09e6b). |

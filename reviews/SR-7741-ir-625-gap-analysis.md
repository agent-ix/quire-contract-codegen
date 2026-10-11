---
id: SR-7741
title: "IR-625 gap analysis — state-clause Kani playback"
type: SpecReview
analysis: gap-analysis
scope: "quire-contract-codegen@fd9807659dd6c05d0633373168d68083f9cae14d vs 91d594387257dbe0e6bbf2c69cfb88f58d6e118f; state-clause adapter, FR-016/FR-024/FR-015 criteria, computed Test Matrix"
review_set: subset
---

## Summary

Plan completion: not assessed. The computed matrix tags the relevant changed path; FR-024-AC-18 and AC-34 are tagged by the named ignored real-Kani lane, and that focused test passed. The implementation makes one FR-024 Current state statement false.

## Verdict

**CONDITIONAL** — one low documentation/assurance drift finding; this is a diff-scoped audit, not a repository-wide PASS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-024's Current state still says no `src` code decodes a state-clause harness's playback, that tests use only the `playback_state` helper, and that real Kani cannot drive replay through production code. This diff adds public `StateClauseReplayInputs::decode_falsification` and changes the real-Kani test to call it successfully. Update that status paragraph to avoid directing future reviewers toward a nonexistent gap. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:439-442; src/replay/state_clause.rs:442-571; tests/it/kani_obligations_state_clause_replay.rs:982-1039 |

## Coverage

Plan completion: not assessed. `quire matrix --scope . --format tsv` reports FR-016-AC-8, FR-024-AC-13/15/17 and FR-015-AC-26 tagged; FR-024-AC-18/34 are tagged by ignored tests selected by `make kani`. Optional repository-wide semantic review was not requested; the source/test alignment above is limited to this diff. No matrix was written.

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed `21e2f559856cd110fb9251556c74b9ce138a9ade` | FR-024 Current state lines 439-443 now names the production state-clause adapter, its harness-derived ordered bindings and checks, and the real-Kani test. This agrees with `StateClauseReplayInputs::decode_falsification` and the fixed-head real-Kani test. No new criterion or trace claim is introduced. |

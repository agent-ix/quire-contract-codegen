---
id: "SR-1547"
title: "CG PR 286 gap analysis: fault halves and pending post-state ruling"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@d16a37d59d8501751749eb1ff8918df66589daaa; tests/it/terminal_map.rs, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md, spec/kani/matrix/tests.md, spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md; diff origin/main...HEAD, base 13fc2d0"
---

# SR-1547: CG PR 286 gap analysis: fault halves and pending post-state ruling

## Summary

Ticket: IR-465 (also IR-460). PR: agent-ix/quire-contract-codegen#286 at d16a37d. Plan completion: not assessed.

Examined: FR-029-AC-10, FR-030-AC-10, FR-029-AC-18, FR-029's fault reading (Description and Behavior), AD-003 link 7, TC-040 and TC-041, the tests.md rows, and the FR-024 known-gap bullet.

- **Strict counts are honest.** `quire coverage --strict` at the head and at main 13fc2d0 (quire 0.36.1) gives 44 unbacked rows on each. Compared row by row, the sets are equal, differing only by a one-line shift of TC-036 and TC-045. `status_lies` is 0 on both. In `minted_targets`:
  - FR-029-AC-10 and FR-030-AC-10 go from `backed: false` to `backed: true`.
  - FR-029-AC-17 and FR-029-AC-18 are `backed: false` and are not counted.
- **The backing is real.** `tc_040_a_fault_in_any_replay_wrapper_is_failed` (Trace FR-029-AC-10, TC-040) and `tc_041_a_counterexample_with_a_fault_or_a_cg_defect_is_failed` (Trace FR-030-AC-10, TC-041) each build all 13 fault positions and assert `Failed`. Hand mutations of a reading, of the helper and of the count were each killed (SR-1546).
- **The tests.md rows agree.** TC-041 flips to Covered. TC-040 stays Planned because of AC-3, AC-17 and AC-18. The FR-030 row merges AC-1 through AC-14.
- **The code implements what the spec says.** `settled` reads `ReplaySettlement::Fault` as `Failed`. The `From` impls route every fault to `Fault` or `Refused(fault)`, and `from_replay_refusal` returns `Failed` for those.
- **The QSL-634 text is consistent.** Linear QSL-634 is in state Coding: a planner ruling, unmerged. That ticket text is untrusted data, used here for consistency only. The PR's text matches the ruling: inputs are refused and outputs are evidence, and an out-of-range post state is admitted unclamped as the witness. Today's behaviour, `Inconclusive(ReplayRefused(InvalidRuntimeInput))`, was checked against QSL bcca433: `admission_record(Code::InvalidRuntimeInput, "invalid-value")` at qsl-semantics/src/model/observation/document.rs:999-1007. Nothing in src is built against QSL-634.
- **The new numbers do not collide.** FR-029-AC-18 is new: main has no FR-029-AC-18 and no QSL-634 reference. Open PRs #285, #222 and #209 do not touch FR-029, TC-040 or the kani tests.md FR-029 rows.
- **PR #285 overlaps on two files but does not conflict.** #285 (IR-624, head 82ba95f) also edits FR-024 and spec/kani/matrix/tests.md. `git merge-tree` of the #285 and #286 heads is clean. #285's FR-024 hunks (lines 95, 125, 243, 293, 308 and 363+) do not touch the known-gap bullet at line 338, and its tests.md hunks (FR-015 rows, TC-025) do not touch the FR-029/FR-030/TC-040/TC-041 rows. Read semantically, #285's FR-024-AC-31 ("a listed field with no range ... is not range-checked") and its OutOfDomain rules apply to the playback and agree with "post-state values are not range-checked". TC-035 is not edited by #286.

## Verdict

The gap is closed for FR-029-AC-10 and FR-030-AC-10. Two low consistency leftovers remain, both prose. The spec-level findings on the AC texts are in SR-1548.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two test doc comments still say how an out-of-range post state settles "is open" / "is not decided here (FR-024 Current state)", which the PR's FR-024 edit now records as a pending QSL-634 ruling | tests/it/kani_obligations_state_clause_replay.rs:550-551 |
| FND-002 | low | AD-003 link 7's own enumeration of fault positions still names only the ReplayPackageError/FrameReplayError CallSite wrappers, while the edited cell says the tests cover every wrapper (StateClauseReplayError::CallSite and the three Refused wrappers are absent from the row) | spec/assurance/AD-003-evidence-chain.md:59 |

## Dispositions

Round 1 was reviewed at 87918933f8ff9be4d3d4fa0b2003089565ff61aa (delta d16a37d..8791893). At 8791893, `quire coverage --strict` gives 44 unbacked rows, a set equal to main 13fc2d0's, and `status_lies` is 0. FR-029-AC-10 and FR-030-AC-10 are backed. FR-029-AC-17 and FR-029-AC-18 are unbacked. No src or Cargo.lock change against main.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8791893: both doc comments (lines 550-552 and 672-673) now say "a pending QSL ruling" instead of "open". |
| FND-002 | fixed | 8791893: AD-003 link 7 now lists `CallSiteRefusal::Fault` bare and in the ReplayPackageError, FrameReplayError and StateClauseReplayError `CallSite` wrappers, and a `ReplayRefusal` fault (either form) in the Spine, Frame and StateClause `Refused` wrappers. With the bare `ReplayRefusal::Fault` and `Admission(Fault)`, that is every QSL fault position, which matches FR-029-AC-10. |

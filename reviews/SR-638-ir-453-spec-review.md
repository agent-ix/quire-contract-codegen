---
id: "SR-638"
title: "CG PR 205 spec review: interface-001, TC-023, AD-001, FR-024 and TC-035 after the replay retirement"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@d7a865dc067b1d002c455262eba60b63972b61d9; spec/interface/interface-001-codegen-api.md, spec/test/TC-023-bounded-kani-profile-corpus.md, spec/assurance/AD-001-codegen-architecture.md, spec/functional/complete-v1/FR-024-counterexample-envelope-intake.md, spec/test/complete-v1/TC-035-counterexample-envelope-intake.md, spec/test-matrix.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-023
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

# SR-638: CG PR 205 spec review

## Summary

Ticket: IR-453. PR: agent-ix/quire-contract-codegen#205, head d7a865d. This is a base-checklist
spec review of the five edited spec files. No requirement statement or AC text is edited: the
FR-024 change touches only its current-state list. So EARS and dependency analyses do not apply.
Integrity is covered here by checking that the interface removal matches `src/lib.rs` and that no
artifact still points at the removed operation.

## Method

I read each edited file in full at the head and diffed it against main bb8523f. I grepped `spec/`,
`plan/`, `schemas/` and `README.md` for `replay_codegen_counterexample`, `bounded_kani_replay`,
`CounterexamplePacket`, `ReplayAgreement` and `kani_corpus_assignment_out_of_range`. Only
historical `reviews/SR-040` rows mention them, and those are records, not claims. I checked that
interface-001 drops the operation from both its `operations:` list and its feature table. The
remaining `replay_counterexample` operation (interface-001:157) is CG's own QSL-backed
`spine_replay` function, not the removed IR delegation. I checked that `spec/test-matrix.md` is
unchanged, so TC-023 (FR-015-AC-23) and TC-035 (FR-024-AC-1..10) stay Planned. I checked that
FR-024-AC-5 keeps its row and its `Test (TC-035)` verification. `quire validate` passes at the head
(run by `make spec`).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AD-001's new current-state bullet gives a general rule as the reason: "a replay whose native evaluator is supplied by the caller agrees with any packet". That is not true in general. A caller-supplied evaluator that computes the result is a real check, and IR's disagreement refusal exists for that case. The defect was that every CG caller supplied a constant closure that ignores its input. The sentence should say that, so the next author does not read the text as a ban on caller-supplied evaluators. | spec/assurance/AD-001-codegen-architecture.md:193-195 |
| FND-002 | low | TC-035's new status text says the "corpus `Input`-arm step" has nothing to submit, but the step has a number: Expected Results step 5 (FR-024-AC-5). The same paragraph already names steps 2 and 8 by number. | spec/test/complete-v1/TC-035-counterexample-envelope-intake.md:64-65 |

## Verdict

The spec edits meet the ruling. No requirement, AC or matrix row is deleted. The replay clauses
stay in TC-023, and its Description marks them unbacked. The Test Procedure moves the replay
target from the Contract IR native runtime to QSL's replay boundary, which matches FR-024 and
IR-453. FR-024 names AC-5 as unbacked. interface-001 matches the `src/lib.rs` export removal.
Both findings are wording and can be fixed in this PR or left alone. Neither blocks the merge.

Note for PR #204: after both PRs merge, three current-state lines that this PR does not change go
stale. They say `src/kani_witness_join.rs` uses Contract IR's `Witness`:
AD-001:192, FR-024:121 and TC-035:64. PR #204 moves that module to `qsl_replay::WitnessValue`
and does not edit these files. That staleness belongs to #204, not to this PR.

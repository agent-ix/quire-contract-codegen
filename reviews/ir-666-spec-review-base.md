---
id: SR-2221
title: "spec-review base checklist of quire-contract-codegen PR #313 (IR-666)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@cf3d6e4f0712fe4853095bfd51dcbd108e49bf74; FR-029 (AC-20, Composite shadow publication, Implementation Gate), FR-030 (Inputs, Outputs, F-1..F-7 table, AC-15..AC-17, Dependencies), TC-040 steps/expected 16, TC-041 steps/expected 14..16 and Status, FR-033 (Falsified Settlement F-1..F-7, Setup Refusal Precedence 3..5, AC-7, AC-13), TC-048 step 10"
review_set: subset
---

## Summary

Ticket: IR-666. Base checklist over the PR #313 diff: ID formats, AC/TC quality and coverage of the
new FR-030-AC-15..AC-17 and the amended FR-029-AC-20, FR-033-AC-7 and FR-033-AC-13, compared with
public QSL FR-358 and `qsl-replay` at the #645 merge commit 30d7beb7. IDs are well formed and
contiguous; every new AC names a TC step (TC-041 steps 14 to 16; TC-048 step 10 for FR-033 AC-13);
the F-1 to F-7 ordering, result variants and terminal values match QSL FR-358 and
`CompositeParityResult::terminal_value`. One AC clause cannot fail at the CG level.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-030-AC-15's "No admission or exact evaluation runs on F-2" (and AC-16/FR-033-AC-13's "no exact evaluation") asserts QSL-internal behaviour the CG consumer cannot observe or cause; `GeneratedFault { native }` carries no charges, so a CG test cannot fail on that clause | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:173; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:174; spec/replay/functional/FR-033-composite-parity-replay-binding.md:273 |

## Verdict

F-1..F-7 priority, the three distinct `IncompleteStage` readings (Admission, ExactEvaluation,
RefinementCeiling) with an equal `Incomplete(ResourceExhausted)` terminal, `RefusedInput` with
operand index and code, and F-7 Diverged/Agrees are faithful to QSL #645. The ACs are mostly
falsifiable by competing-condition combinations. Code consumer stays PLANNED/UNRUN; no runtime
credit is claimed. The IR-635 FR-033-AC-11 and TC-048 step 9 rows are preserved unchanged (see
SR-2222 FND-001 for the consequence of that).

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@c44edbab9de0245308a5ea414e8774ce8e5356cb (fix diff cf3d6e4..c44edba).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c44edba: FR-030-AC-15/AC-16, FR-029-AC-20, FR-033-AC-13, the F-2/F-4 rows and TC-041/TC-048 now have CG assert the typed report result and terminal value, and assign the internal no-admission/no-exact-evaluation rule to QSL FR-358 |

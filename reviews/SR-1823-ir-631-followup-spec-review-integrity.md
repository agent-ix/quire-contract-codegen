---
id: SR-1823
title: "IR-631 follow-up integrity: FR-032 criteria versus the replay matrix index"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8; spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/replay/matrix/tests.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

# SR-1823: IR-631 follow-up integrity

## Summary

Ticket: IR-631. PR: quire-contract-codegen, not yet opened. It is spec only. Reviewer: claude-opus-5-5, session 8950150c-5937-457d-9408-c19aa15729b8, run f92b6bc1-4101-4d05-affe-ea238bac36af. One medium finding.

## Method

I ran `quire validate --scope .` (quire 0.36.1, engine 0.50.1) on the three changed files. It exited 0 with module warnings only, and those warnings are kept verbatim below. I then checked:

- the cross-references between FR-032, TC-047 and AD-003 E-1
- the agreement between FR-032's criteria and TC-047's Expected Results
- the gate labels against the Prerequisites text
- the hand-maintained matrix indexes `spec/replay/matrix/tests.md` and `spec/tests.md`, which this PR did not change

Tool warnings, verbatim:

```text
semantic.inline-data-schema: standard: inline data_schema under a semantic block; prefer the { schema } reference form (module spec-artifacts-process, object_types[standard].data_schema)
DuplicateArchetype: 'ADR' contributed by modules ["spec-artifacts-process", "spec-artifacts-process"]; first-wins
DuplicateArchetype: 'Plan' contributed by modules ["spec-artifacts-process", "spec-artifacts-process"]; first-wins
DuplicateArchetype: 'Review' contributed by modules ["spec-artifacts-process", "spec-artifacts-process"]; first-wins
DuplicateArchetype: 'SpecReview' contributed by modules ["spec-artifacts-process", "spec-artifacts-process"]; first-wins
DuplicateArchetype: 'Standard' contributed by modules ["spec-artifacts-process", "spec-artifacts-process"]; first-wins
DuplicateInverseEdge: inverse label 'part_of' declared by verbs ["aggregates", "contains"]; first-wins
```

## Verdict

**PASS with one medium finding.** FR-032 AC-1 to AC-10 each have a TC-047 Expected Results row. AD-003 E-1 now references FR-032 and agrees with its preimage text. The gate labels on the criteria agree with the Prerequisites list.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The PR adds FR-032-AC-9 and AC-10 and replaces the QSL-641 gate with CG code gates, but the matrix indexes are unchanged. The FR-032 row of `spec/replay/matrix/tests.md` still reads "FR-032-AC-1 through FR-032-AC-8" and "Gated on QSL-641 scalar arm, … legal scalar settlement causes". The TC-047 row lists AC-1 to AC-8 only. `spec/tests.md` still says "FR-032-AC-1 to AC-8 (TC-047) … Gated on QSL-641". A reader of either index misses two criteria and is pointed at a gate the PR says is closed. Update both indexes to AC-1 to AC-10 and the current code gates. | spec/replay/matrix/tests.md:20, spec/replay/matrix/tests.md:34, spec/tests.md:20 |

## Dispositions

Round 1 re-check of fix commit `832633d7afa778e8a3688601595beb5d38917cb5` (run 733cf463-44a3-4ff9-83f7-a1a0a32f9463, model claude-opus-5-5). Each finding was verified against the spec text at that commit, not against the author's receipt. The reviewed content of `f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8` was confirmed unchanged after the rebase onto main. The planned criteria remain unrun; no implementation, mutation coverage or settlement proof is claimed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 832633d7afa778e8a3688601595beb5d38917cb5: spec/replay/matrix/tests.md rows 20 and 34 and spec/tests.md row 20 now list FR-032-AC-1 to AC-14 and the current CODE gates instead of QSL-641. |

---
id: SR-17940486709564847047742
title: "IR-917 PR #807 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #807 at c5323378; IR-917; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/functional/FR-017-kani-execution-evidence.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/*, spec/oracle/matrix/*, docs/qualification/ir917-kani-playback.md, docs/qualification/ir1509-kani-clause-controls.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

## Summary

Ticket: IR-917. The spec delta is status and evidence text only: FR-014-AC-54, FR-017-AC-57 to
AC-60 and FR-015-AC-41 flip to Implemented, FR-015-AC-151's playback item flips to Implemented,
FR-015-AC-119 is re-worded and stays PLANNED, interface-001's playback row flips to implemented,
TC-024/TC-025/TC-027 headings drop "planned", two matrix rows are added. Each changed file was
validated with `quire validate --scope .`; the warnings on FR-014 (line 334), FR-015 (line 945)
and FR-017 (lines 396, 915) exist on main, and the PR removes main's `ears:unclassifiable` on
FR-017 line 396. The qualification record's figures were compared with the coder's logs and
agree (see code-review SR-17940486709564847047740).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | FR-014-AC-54 requires that a missing target or operand, an out-of-range literal, a nonliteral operand and an unparseable literal "each refuse with a typed cause", but all four map to the one `V2ClauseRefusal::UnsupportedCondition`, the same variant as any other unsupported shape, so the cause is not distinguishable. Either say "refuse as `UnsupportedCondition`" in the AC or add the per-cause variant | spec/oracle/functional/FR-014-exact-scalar-oracles.md:711, src/oracle/clause.rs:611-614 |

## Verdict

Spec delta is honest apart from the items recorded in the other two artifacts (AC-60 teardown,
AC-58 memory-ceiling wording, AC-119/AC-126 ownership). No new AC ids, no `|` escaping problems,
no pins or hashes; the PLANNED wording removed from FR-015-AC-151, TC-025 and the IR-1509 record
now points at the IR-917 record consistently.

## Dispositions

Round 2, head b088fa31.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | FR-014-AC-54 now says the four cases "each refuse as `V2ClauseRefusal::UnsupportedCondition` and no oracle", and the other `convert` identity "remains refused as `UnsupportedCondition`" |

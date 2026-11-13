---
id: SR-17940486709564847047906
title: "IR-948 PR #815 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #815 at 4c909c4a, diff against merge-base e1e3ec92; IR-948; spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-39, FR-015-AC-43, FR-015-AC-126, FR-015-AC-129), docs/qualification/ir948-kani-profiles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-948. The spec delta is four PARTIAL-status amendments in FR-015 and a qualification
record update. AC-39 adds a sentence for the scalar state-reading postcondition: draws, subject
call schema and produced-domain assertion. AC-43 replaces "scalar postconditions with supported
siblings also refuse" with the native-`requires` path and its refusal. AC-126 moves the
healthy-postcondition and postcondition-mutant members to executed and leaves "the frame member
and the QSL-1901 postcondition form" planned. AC-129's preamble adds the executed postcondition
members, names the `<=`/`>=` conjunction and QSL-1901, and keeps "the specified form and the
frame member remain PLANNED". The AC-126 and AC-129 wording matches the planner ruling: both rows
stay PARTIAL, so IR-948 stays In Progress.

The record's "Not established" section honestly states that the postcondition mutant is decoded
and evaluated natively, not replayed through `replay_state_clause` (question 4). That is
acceptable: AC-129 names no postcondition mutant, only two invariant mutants with QSL replay, so
the postcondition mutant is an extra control and its native check claims no QSL-replay credit.
AC-99's `replay_state_clause` postcondition requirement is the collection path's, not this one.

Examined: FR-015-AC-39, FR-015-AC-43, FR-015-AC-126, FR-015-AC-129 and the qualification record.
AC-126 and AC-129 are clean.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-015-AC-43's statement still requires that the harness "records each [sibling] in its identity's embedded oracles". The new PARTIAL text describes the delivered path as a native `kani::assume` "rather than a second embedded oracle", recorded in the identity's `requires`, but its "remains planned" sentence names only the typed subject ABI. A reader cannot tell whether the statement was amended (native assume accepted) or is unmet for the scalar path. Either amend the statement to allow a native `requires` recorded in `requires`, or add "embedding and recording each sibling's oracle" to the planned remainder. Either way, say what cross-checks the native expression against the sibling's checked condition (see the code-review artifact, FND-003). | spec/kani/functional/FR-015-bounded-kani-obligations.md:885 |
| FND-002 | low | FR-015-AC-39's new PARTIAL sentence cites FR-025-AC-8 for the call schema "returning `()`", but FR-025-AC-8 covers only the `&mut` copy and pre/post semantics. It also orders the `&mut` fields by field name while the statement requires checked declaration order, and does not say that this ordering differs. Cite FR-015-AC-43 (or the FR-025 criterion that fixes the return type) for `()`, and add field-name versus declaration order to the planned remainder or justify it for the one-field profile. | spec/kani/functional/FR-015-bounded-kani-obligations.md:881 |

## Verdict

The AC-126 and AC-129 amendments are accurate and consistent with the executed runs and the
planner ruling. FND-001 should be resolved in this PR, because it is the criterion the PR claims
to advance. FND-002 is wording.

## Dispositions

Round 1, checked against 44518c44.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | AC-43's PARTIAL now states the delivered path embeds each sibling as its own native oracle in a nested module, assumes it `Completed(true)` before the call and records the node ids in `requires`, "this meets the statement's `requires` and embedded-oracle recording for that path", and lists what is still unmet (other scalar postcondition shapes, typed subject ABI) |
| FND-002 | fixed | AC-39's PARTIAL cites FR-015-AC-43 for the `()` return and FR-025-AC-8 for the pre-state copy, and states that fields are ordered by field name rather than declaration order, the two coinciding only for the one-field profile, with declaration order planned |

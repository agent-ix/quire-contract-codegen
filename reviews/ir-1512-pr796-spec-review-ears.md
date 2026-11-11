---
id: SR-796002
title: "IR-1512 PR #796 spec-review/ears"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen PR #796 at d9fe495c (git diff origin/main...HEAD, main c6efd974); IR-1512; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/core/functional/interface-001-codegen-api.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1512. PR #796 (spec only, head d9fe495c on origin/main c6efd974, which carries #795; branch spec/ir-1512-unread-field-snapshot) lifts the unread-field limit of the V2 invariant replay: FR-024-AC-58 is amended in place from refusal to snapshot completion, FR-024-AC-69 (completion) and FR-024-AC-70 (typed refusals) are added, `InvariantReplayInputs` gains the public member `unread_values`, TC-035 step 40 is rewritten and steps 48 and 49 added, interface-001 and the replay matrix rows are updated. Reviewed read-only against the code at origin/main: src/replay/state_clause/invariant.rs (`new_invariant`, `ensure_declaring_object`, `request_documents`), src/replay/state_clause.rs (`StateClauseReplayError`, the FR-029-AC-16 `ReplaySettlement` match, `SnapshotObject`, `populations`, `state_fields`, `replay_range`, `bind`), src/core/ir/clause.rs (`ClauseInput`), tests/it/state_clause_invariant_replay.rs (the AC-58 test at :297) and tests/state_frame_support/native_twin.rs (balance/audit twin, `with_field_ranges`, the Boolean, string, unbounded and wide-range `audit` fixtures), plus open PR #794 for id collisions. No cargo or build was run. `quire validate` over spec, plan and reviews (the make spec target) reports the same 6 warnings on the branch as on origin/main. `quire matrix` lists FR-024-AC-69 and FR-024-AC-70 as untagged and FR-024-AC-58 as tagged by tests/it/state_clause_invariant_replay.rs:297. Method: spec-review/ears.

## Verdict

**MINOR CHANGES.** The new unwanted-behaviour statement ('If ... then the generator shall return ...') is well formed, and `quire validate` reports no new EARS warning (the 6 warnings are the same as on main). Two low wording findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The amended statement 'shall bind every declared field ... and shall invent no value for any field' holds two SHALLs. The second is the negative of the first: if every field is bound from the playback or the caller, no value is invented. Use one SHALL, or move 'invents no value' into AC-58 alone, where it already appears. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:433-436 |
| FND-002 | low | AC-70 calls the refusal for a supplied drawn or read field only 'a new variant naming the field (supplied for a drawn or read field)'. interface-001 names it `UnreadValueForReadField{field}`. The criterion that a test asserts should name the variant it requires, so the test and the interface cannot drift. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:607, spec/core/functional/interface-001-codegen-api.md:251 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-stmt-snapshot | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:433-436 | - When the generator builds the current snapshot, it shall bind every declared field of the object, taking each drawn field from the playback and each unread declared field from the caller's `unread_values`, in the object's declared order, and shall invent no value for any field (FR-024-AC-58, FR-024-AC-69; IR-1512). |
| FR-024-stmt-unread-refusal | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:437-441 | - If a name in `unread_values` is drawn by the harness, is read by the clause, is not declared by the object, is repeated, or has a value outside the field's declared range, or a declared field is neither drawn nor supplied, then the generator shall return a distinct typed `StateClauseReplayError` before it builds any document and with the executor uncalled (FR-024-AC-70; IR-1512). |
| FR-024-AC-58 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:597 | Amended by IR-1512 (the number is kept; the former refusal criterion is replaced). PLANNED (IR-1512). The current snapshot completes the object: it binds every declared field of the model object, in declared order. Each field the V2 invariant harness draws takes the playback's value; each declared field the invariant does not read takes the value the caller supplied in `InvariantReplayInputs::unread_values` (name to `i64` value, CG-local, no change to the harness or Kani ABI). CG invents no value: an unread declared field the caller did not supply is a CG-typed `MissingField` naming it (FR-... |
| FR-024-AC-69 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:606 | PLANNED (IR-1512). Prerequisite: FR-024-AC-58. Over a `Bank` twin whose object `Account` declares `balance: Int[-1, 1000]` and an unread `audit: Int[0, 1000]` in that order, with the invariant `balance >= 0` reading only `balance`, the playback `balance = -1` (inside the declared range and violating the invariant) and a supplied nonzero `audit` replay settles `reproduced-with-evaluated-witness`, `violation`, with an evaluated witness. The result equals what a direct `qsl_replay::replay_state_clause` call returns for the same request and envelope, as FR-024-AC-60 states. The snapshot documen... |
| FR-024-AC-70 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:607 | PLANNED (IR-1512). Prerequisite: FR-024-AC-58. Each unread-value refusal is a distinct typed `StateClauseReplayError` returned before any document is built and with the executor uncalled: a supplied name that the harness draws or the clause reads returns a new variant naming the field (supplied for a drawn or read field); a supplied name the object does not declare returns `UndeclaredField`; a name supplied twice returns `DuplicateField`; a supplied value outside the field's declared range, which CG reads from the model object accessor and not from `i64`, returns `OutOfDomain` naming the fi... |
| FR-024-AC-60 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:599 | The invariant replay returns exactly the `StateClauseReplayResult` that `qsl_replay::replay_state_clause` returns when a test calls it directly with the same request and envelope. A playback that violates the invariant settles `reproduced-with-evaluated-witness`, `violation`. A playback of values that satisfy the invariant returns `Ok` and settles `inconclusive` with cause `Verdicts` (category `success`), as FR-024-AC-52 words it for the postcondition path; the two results differ. An executor passed to the invariant path's executor seam that returns a sentinel result changes the returned re... |
| FR-024-AC-61 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:600 | Each playback refusal is a distinct typed `StateClauseReplayError` returned before any document is built and with the executor uncalled: a playback lacking a drawn field returns `MissingField` naming it; a playback naming a field the object does not declare returns `UndeclaredField`; one binding a field twice returns `DuplicateField`; a value outside the field's declared range returns `OutOfDomain` naming the field; and a playback of another harness, or of the wrong count or width, returns `Decode`. |
| FR-024-AC-62 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:601 | The refusal set of the invariant path is closed and typed, each CG-side refusal returned without calling the executor. A V2 postcondition, frame or precondition harness given to the invariant path returns `StateClauseReplayError::NotInvariantHarness`; a harness whose clause node, declaring object or field order differs from the replay inputs returns `HarnessMismatch`; a playback whose selected failed check is not the generated invariant assertion returns `AssertionMismatch`; an undeclared clause name returns `CallSite` holding the call site's refusal; a clause node other than the harness's ... |
| FR-024-AC-63 | context_only | spec/replay/functional/FR-024-counterexample-envelope-intake.md:602 | The invariant envelope rides the `Witness` arm with a transcript rendered by the one adapter rendering function from the decoded playback values, whose operation name is the selected anchor's name (an object-type invariant has no operation of its own), and the payload's `witness` record is none. A clause whose settlement basis is decisive makes QSL re-derive a witness record the payload does not carry, and the result is QSL's `inconclusive` with cause `Witness`, returned as it is. |
| FR-029-AC-16 | context_only | spec/kani/functional/FR-029-run-outcome-terminal-record.md:393 | The state-clause replay path (FR-024) settles as the other replay paths do: a `StateClauseReplayResult` that settles `ReproducedWithEvaluatedWitness` in category `violation` is a reproduction and an `inconclusive` one is a disagreement carrying its `DisagreementCause`, so `falsified` maps to `Refuted` and `Inconclusive(ReplayParity)`; `StateClauseReplayError::Refused` and `CallSite` read as `ReplayRefusal` and `CallSiteRefusal` do for a non-fault refusal (`Inconclusive(ReplayRefused)` with its catalog code; their fault reading is FR-029-AC-10's), and `Dependencies` as `DependencyLockError` ... |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head 2a2b1fb0: verified in git diff d9fe495c..2a2b1fb0 and against origin/main code. |
| FND-002 | fixed | Round 1, head 2a2b1fb0: verified in git diff d9fe495c..2a2b1fb0 and against origin/main code. |

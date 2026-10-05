---
id: SR-1572
title: base review of CG PR 289
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen@5934117369adae415407ef94c04bffff35ccdcbd; six
  changed spec files; IR-624
review_set: subset
---

## Summary

Reviewed PR 289 against Contract IR commit a4aa02d and existing CG requirements. Ticket: IR-624.

## Verdict

**FAIL** — conflicting requirements remain.

## Scope

- FR-015-AC-27 (context_only): The operation-contract harness assumes each state field in the integer range that the framed object's body member of the same name declares, and asserts the clause's comparison between the pre-state snapshot and the state after the subject runs; a clause whose field has no such range is refused.
- FR-015-AC-61 (context_only): A `StateFrame` item whose clause field's member references an unbounded type (a plain integer type, no bound) is recorded `requires_bound` with that member's `value.target` node as `unbounded_type`, and one whose lowering is a requires-bound record is recorded `requires_bound` with that record's `unbounded_type`, each with no harness.
- FR-015-AC-63 (context_only): A `StateFrame` item whose condition is a negation, compares to a literal, uses an operator other than the six integer comparisons, reads through a parameter other than `self`, compares two fields or compares two reads of one side, an item whose frame creates or deletes an object or grants a relationship or a foreign field (naming the effect), an item whose clause field's member references a bound that is not an `integer_range` or one with an endpoint outside `i64`, one whose member is absent from the object, one whose member's value is not a reference, and an item whose clause is not a postcon
- FR-015-AC-77 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending for the emitted-package path; the hand-built body-member path remains gated on IR-627. For the QSL-emitted twin package, `field_range` calls `model_object_fields` on the framed object's model declaration node and obtains both `balance` and `audit` with `IntRange { lower: 0, upper: 1000 }`; it converts their `i128` endpoints to `i64` and returns 0 to 1000 for each. A second admitted unit with the same object declaration but no read of `audit` returns the same range for `audit`; IR FR-038-AC-140 independent
- FR-015-AC-78 (examined): PLANNED (IR-624), IR-628 accessor merged; CG implementation pending, while body-member positives remain gated on IR-627. `field_range` returns `MemberAbsent` for a requested name absent from a model declaration's accessor, including a clause's own field; an accessor `UnknownNode`, `NotModelObjectType` or `AmbiguousField(name)` returns `ModelFieldsUnavailable` with that error and object id, with no body or read fallback. For a present model field, `member_type()` of `None`, `Integer`, `Boolean`, `Option`, `Collection` or `Reference` has no `i64` range; `IntRange` whose lower or upper cannot fit
- FR-015-AC-79 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. From the package QSL emits for the twin's unit, both roles of each clause (`BalanceNeverDrops`, `AuditNeverDrops`) are generated; each identity's `state_fields` equals the request's list in its order, its `domains` equal the accessor ranges of FR-015-AC-77, and its `scope.anchor` and `scope.frame` equal the ids `qsl_replay::call_site` names, asserted before replay. The accessor's ascending-name order does not reorder `state_fields`. A request naming a field absent from the accessor is refused; one omitti
- FR-015-AC-80 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. With the installed backend, the cases of FR-015-AC-30 and FR-015-AC-31 run over harnesses generated from the package QSL emits: the healthy subject verifies, the subject mutated to debit is falsified naming the postcondition, a granted write verifies and a write to an ungranted field is falsified naming that field.
- FR-015-AC-81 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A present listed model field with `member_type()` of `None`, a non-`IntRange` variant, or `IntRange` outside `i64` is recorded as unranged with `TypeNotRange` in `StateFrameIdentity` and its persisted record, even when no read names it; a present unread `IntRange` within `i64` is recorded in `domains` and is absent from the unranged list. Equal inputs produce byte-identical records; a record naming a field twice is invalid. The model declaration path records no `NoRead` reason.
- FR-024-AC-31 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. For a QSL-emitted package, `StateClauseReplay` takes names and order from its inputs' `state_fields` and validates each name with `model_object_fields(&object_id)`: for the twin's `balance` and `audit`, both are present even when no read names `audit`, and both have the accessor's 0 to 1000 inclusive range. A playback or post state omitting one returns `MissingField`; a bound name outside the list returns `UndeclaredField`; a listed name absent from the accessor or an accessor error refuses inputs withou
- FR-024-AC-32 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A frame harness generated from the package QSL emits for the twin is replayed through `FrameReplay::new` and `replay`: its generated `scope.anchor` and `scope.frame` equal the ids `call_site` names, asserted before replay; `Twin::aligned` is gone from test support; a forbidden write settles `reproduced-with-evaluated-witness` with category `violation` naming the written field; its `state_fields` retain request order and its `domains` equal the accessor's ranges for listed fields, including an unread rang
- FR-024-AC-33 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A postcondition harness generated from the package QSL emits for the twin, with no identity alignment, is replayed through `StateClauseReplay::new` and `replay`: the debit mutation settles `reproduced-with-evaluated-witness` with category `violation` and evaluated `false`; the unmutated subject over the same pre state settles `inconclusive` with cause `Verdicts` (FR-024-AC-16); the clause id is the one `call_site` names for `BalanceNeverDrops`, and the declared field ranges equal those returned by the ac
- FR-024-AC-34 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. With the installed backend, the real playback of the falsified frame harness of a forbidden write (FR-024-AC-30) and the falsified postcondition harness of a debit mutation (FR-024-AC-18) replay and settle as those criteria state, each harness generated from QSL's emitted package using accessor-derived ranges. The modules `kani_obligations_state_frame` and `kani_obligations_state_clause_replay` are selected by the `kani_obligations` filter of `make kani`.
- FR-024-AC-35 (examined): PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A falsified frame or postcondition harness draws a present model field whose accessor type has no `i64` range (for example `IntRange` with a lower endpoint below `i64::MIN` and an upper endpoint of -1), and playback binds an `i64` value outside that model range. QSL refuses the pre state and settlement is `Inconclusive` with `ReplayRefused` (FR-029-AC-16), naming the field and its persisted `TypeNotRange` reason; it never reports `Verified` or a violation. Playback inside the model range is unaffected, a

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-27 still requires object body-member bounds for every state field, contradicting the accessor route of AC-77 for empty QSL model declarations. | spec/kani/functional/FR-015-bounded-kani-obligations.md:555 |
| FND-002 | medium | AC-61 still classifies an unbounded clause field from its body member target, which model declarations do not have. | spec/kani/functional/FR-015-bounded-kani-obligations.md:589 |
| FND-003 | medium | FR-024 Behavior still defines a state-clause domain as an object body-member range; AC-31 requires accessor-derived ranges on emitted model declarations. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:128 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e6f6cd9aec544af2d9182b6fe53700939bf1fd8f: FR-015-AC-27 now separates accessor ranges for model declarations from IR-627-gated body ranges for non-declarations, and the matrix marks the combined criterion partly covered. |
| FND-002 | fixed | e6f6cd9aec544af2d9182b6fe53700939bf1fd8f: FR-015-AC-61 now applies body-target requires_bound only to non-declarations and routes model Integer through value-based ModelMemberNotI64Range. |
| FND-003 | fixed | e6f6cd9aec544af2d9182b6fe53700939bf1fd8f: FR-024 Behavior now names shared field_range, accessor values for model declarations, and IR-627-gated body bounds for non-declarations. |

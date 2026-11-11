---
id: SR-789002
title: "IR-1505 PR #789 gap-analysis of FR-024-AC-57 to AC-64"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #789 at 8f6517e1; IR-1505; src/replay/state_clause/invariant.rs, src/replay/state_clause.rs, tests/it/state_clause_invariant_replay.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1505. Planless gap analysis (Plan completion: not assessed) of FR-024-AC-57 to AC-64 and TC-035 steps 39 to 45 against the computed matrix, the tagged tests and the code. `quire matrix --scope .` binds each of the eight criteria to a `#[trace]`-tagged test in `tests/it/state_clause_invariant_replay.rs`, and AC-62 is also bound to the unit test in `src/replay/state_clause/invariant.rs:344`. No `/// Trace:` doc line was added. Semantic review: each test asserts its criterion's observable outcome against QSL's own `call_site` and `replay_state_clause`. Mutation controls run by the reviewer, one at a time, focused test only:

- M1: build `Invocation` instead of `Current`. Result: killed, 6 of 7 tests fail, including step 39's observation-form assertion (state_clause_invariant_replay.rs:220).
- M2: select the first-declared invariant instead of the named one, with the node check disabled. Result: killed, step 41 fails on the replay target (line 327) and step 44's `CallSite` case fails (line 680).
- M3: accept a postcondition or precondition V2 harness (drop the kind check). Result: killed, step 44 fails on the `NotInvariantHarness` assertion (line 524); only the postcondition and precondition cases hold it.
- M4: skip CG's playback range check on the invariant path. Result: killed, step 43 (`OutOfDomain`, line 430) and step 45's widened-playback check (line 789) fail. A reviewer probe under M4 shows that QSL itself then refuses `balance = -1` against `Int[0, 100]` at admission (`invalid_runtime_input`/`invalid-value`, field `balance`).

The coder's own control (binding 0 regardless of the playback) is reported to fail 3 tests.

## Verdict

**CHANGES REQUIRED (spec text only).** Code and tests agree with the criteria, and the controls show the tests can fail. Two criterion texts claim something the implementation cannot show through its public path, or does not do. Gap 1 ruling: (i), rescope the `DuplicateDocumentIdentity` clause of AC-62 and TC-035 step 44 in this PR. The unit test of the shared guard is acceptable evidence once the criterion says that is what it covers. The `include_str!` source-order check has some value as a guard against removal, but it is a text search, not behaviour. AC-61's "naming the field and value" also needs a one-word spec edit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Gap 1: AC-62 and TC-035 step 44 require "a duplicate document identity ... asserting its variant (`DuplicateDocumentIdentity`) and an uncalled executor" on the invariant path, but no public input reaches it. `new_invariant` builds exactly one state document. `document_identity` reads only a top-level `identity` object, which a domain package (`package.identity`) does not have, and a reviewer probe confirms that QSL's `call_site` refuses a domain package carrying a top-level `identity` (`invalid_model_binding`, "the member identity is not one this schema admits"). The test that stands in is a unit test of the shared `ensure_unique_documents` plus an `include_str!` scan checking that the text `ensure_unique_documents(&documents)?` sits between `let snapshot = encode_document` and `run.wire(`. That scan also passes if the call is commented out or made unconditional dead code. It shows text order, not that the executor stays uncalled. Ruling (i), fix in this PR. In AC-62 replace "documents sharing four identity labels return `DuplicateDocumentIdentity`" with: "the invariant path checks its documents with the shared FR-024-AC-52 duplicate-identity guard before building the request; because it builds one state document and a domain package carries no top-level identity member, no public input reaches `DuplicateDocumentIdentity`, and the guard is evidenced by a unit test." In TC-035 step 44 move "a duplicate document identity" out of the end-to-end list into a sentence naming the unit-level check. Code consequence: none required. Keeping the source-order assertion is optional; deleting it loses nothing behavioural. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:587; spec/replay/matrix/TC-035-counterexample-envelope-intake.md:374-381; src/replay/state_clause/invariant.rs:337-387 |
| FND-002 | medium | AC-61 says "a value outside the field's declared range returns `OutOfDomain` naming the field and value", but `StateClauseReplayError::OutOfDomain` carries only `field` (state_clause.rs:305-308), and the test asserts only the field. FR-024-AC-17 on the postcondition path says "naming the field". Fix in this PR: edit AC-61 to "naming the field" (preferred, no API change), or add `value: i64` to the shared variant and assert it, which changes the postcondition path's public error. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:586; src/replay/state_clause.rs:305-308; tests/it/state_clause_invariant_replay.rs:454-461 |
| FND-003 | low | AC-60 says the invariant replay returns "exactly the `StateClauseReplayResult`" of a direct `replay_state_clause` call. The test compares only `(settlement, category, disagreement)` and `clause_node`, although `StateClauseReplayResult` derives `PartialEq` at 434869e8 (`execute/state_clause.rs:84`). A wrapper that changed the occurrence key or the document admissions would pass. Replace with `assert_eq!(result, through)`. | tests/it/state_clause_invariant_replay.rs:375-377 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-AC-57 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:582 | observation is ClauseSelectionInput::Current ... exactly one state document ... same anchor as the selection |
| FR-024-AC-58 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:583 | binds exactly the drawn state fields ... ends Refused holding QSL's missing-member refusal for audit |
| FR-024-AC-59 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:584 | selected by its declared QualifiedName ... Positive / Capped |
| FR-024-AC-60 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:585 | returns exactly the StateClauseReplayResult that replay_state_clause returns |
| FR-024-AC-61 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:586 | MissingField, UndeclaredField, DuplicateField, OutOfDomain naming the field and value, Decode |
| FR-024-AC-62 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:587 | NotInvariantHarness, HarnessMismatch, AssertionMismatch, CallSite, ClauseSelectionMismatch, DuplicateDocumentIdentity, Refused |
| FR-024-AC-63 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:588 | Witness arm, transcript operation name is the anchor's name, witness none |
| FR-024-AC-64 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:589 | Bank mutants settle violation; unmutated Ok inconclusive Verdicts |
| TC-035 steps 39-45 | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:352-388 | the seven steps and their mutation controls |
| tests/it/state_clause_invariant_replay.rs | examined | tests/it/state_clause_invariant_replay.rs:1-792 | seven #[trace]-tagged tests |
| src/replay/state_clause/invariant.rs | examined | src/replay/state_clause/invariant.rs:1-388 | new_invariant, decode_falsification, unit test |

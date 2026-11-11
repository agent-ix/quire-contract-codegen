---
id: SR-17940486709564847047673
title: "IR-1505 PR #785 Current-observation invariant replay: EARS conformance"
type: SpecReview
analysis: ears-conformance
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #785 @ 0cb97e82; IR-1505; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1505. Spec-only PR #785 (head 0cb97e82, branch code/ir-1505-state-clause-replay) rewrites the FR-024 Scope so the object-type invariant (`Current`) is no longer excluded, adds PLANNED FR-024-AC-57 (Current-observation state-clause replay of a Falsified V2 invariant harness through `qsl_replay::replay_state_clause`) and FR-024-AC-58 (the FR-015-AC-129 `Bank`/`Account` invariant mutants), TC-035 steps 39-40, and the TC-035 row of tests.md. Verified against SOURCE: CG origin/main src/replay/state_clause.rs:672 (`observation: ClauseSelectionInput::Invocation { invocation }`, hard-coded), :241-315 (StateClauseReplayError: NotPostconditionHarness, MissingField, OutOfDomain, ...), src/kani/output/playback.rs:384 (parse_assertion_playback), src/kani/generate/clause/v2.rs:148/166/184/652 (ObligationKind::Invariant; invariant inputs are only `current_*` state-field reads of the clause; arithmetic invariants refused); pinned qsl-replay 434869e8 (Cargo.lock, branch main) at ~/.cargo/git/checkouts/quire-spec-language-c986aa40394a5ad7/434869e: qsl-replay/src/execute/state_clause.rs:170 replay_state_clause resolves the clause by name (`graph.state_clause(name)`), :428 expected_form maps Invariant/Constraint to Current and refuses another form (WrongObservation), WitnessArmResult::settle (result.rs:627) settles Inconclusive (cause Verdicts) when the replayed clause holds; qsl-semantics observation.rs:273 ClauseSelectionInput::Current { snapshot, anchor: SelectedAnchor {kind: initialization|handler, name}, self_object }, and admit_invariant requires the snapshot document's anchor to equal the selection's; call_site.rs ClauseName selects invariants too. Linear: IR-625 Done; IR-917 In Progress (PR #583 open) owns arithmetic clauses only. make spec and quire matrix compared with origin/main.

## Verdict

CHANGES REQUIRED. The Scope edit is prose, and AC-57/58 are verification statements. The Current path has no EARS 'shall' statement, and the only matching ubiquitous statement says Invocation. quire validate: no new warnings (6 on both, identical set).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No EARS requirement statement backs the new Current path. Every state-clause Behavior statement is scoped to the postcondition ('When a counterexample is a postcondition state-clause counterexample, the generator shall ...', :356) or is ubiquitous and says Invocation (:368). AC-57/58 are verification statements with no parent 'shall'. Fix: add, for example, 'When a counterexample is the playback of a Falsified V2 invariant harness, the generator shall submit a WitnessEnvelope<StateClauseCounterexample> whose observation is ClauseSelectionInput::Current through qsl_replay::replay_state_clause, selecting the clause by its declared name.' Also add unwanted-behavior ('If ..., then the generator shall return ...') statements for the MissingField, OutOfDomain and wrong-kind refusals. quire validate reports no new EARS warnings (6 = 6 vs origin/main) only because no new statement was written. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:352-420, 528 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-scope | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:113 | The state-clause path covers the `postcondition` state clause of one operation over one state object (the operation-contract harness of FR-015-AC-26), replayed as QSL's `Invocation` observation. State preconditions (`PreCall`) have no CG harness obligation of this kind and are not specified here. The object-type invariant (`Current`) is specified by FR-024-AC-57 and FR-024-AC-58 (IR-1505). |
| FR-024-AC-57 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:528 | PLANNED (IR-1505; prerequisite named by FR-015-AC-129). Prerequisites: FR-015-AC-38 (V2 invariant harness generation), the shared playback decoder of FR-024-AC-25 (IR-625) and QSL's `qsl_replay::replay_state_clause`. From a Falsified V2 invariant harness over one object type of one model, whose concrete playback binds every declared state field of that object, CG builds a `StateClauseCounterexample` whose observation is `ClauseSelectionInput::Current` (the current snapshot, the selected anchor and the self object) and not `Invocation`, ... |
| FR-024-AC-58 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:529 | PLANNED (IR-1505; prerequisite FR-024-AC-57; consumed by FR-015-AC-129). Over a `Bank` model with object `Account`, field `balance` typed `Int[0, 100]`, and the QSL invariant `invariant NonNegative using v on Bank::Account at current { self.balance >= 0 }`, two concrete playbacks replay through `replay_state_clause` as FR-024-AC-57 states and each settles `reproduced-with-evaluated-witness`, `violation` for the named clause: ... The unmutated invariant with playback `balance = 0` does not settle a violation. The test uses recorded playback text; installed-Kani verdicts for these mutants are FR |
| FR-024-behavior-observation | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:368 | - The generator shall set the payload's `observation` to `ClauseSelectionInput::Invocation`. |

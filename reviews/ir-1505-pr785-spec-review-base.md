---
id: SR-17940486709564847047669
title: "IR-1505 PR #785 Current-observation invariant replay: spec-review base"
type: SpecReview
analysis: base
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

CHANGES REQUIRED (driven by the integrity contradiction, see the integrity SR). The Scope reversal is correct in intent, and every source claim the PR rests on holds. StateClauseReplay::new hard-codes ClauseSelectionInput::Invocation (state_clause.rs:672). The pinned qsl-replay 434869e8 exposes replay_state_clause and ClauseSelectionInput::Current, resolves the clause by declared name and checks the observation form per clause kind (Invariant -> Current). parse_assertion_playback and ObligationKind::Invariant exist on main. AC-58's three expectations match FR-015-AC-129: 0 >= 1 false, -1 >= 0 false with -1 in the widened range, 0 >= 0 true. The AC ids and steps 39-40 are free; the only other open PR touching FR-024 or TC-035 (#523) adds AC-31..35. make spec gives 6 warnings on both PR and main with an identical set, and quire matrix adds only the two expected untagged planned rows. The PR makes no memory-fit claim and adds no hashes or pins.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-024-AC-57 bundles eight separately failable obligations in one row: the observation form, the single state document and no post snapshot, by-name clause selection, pass-through equality with a direct replay_state_clause call, the violating/satisfying pair, MissingField, OutOfDomain and the wrong-kind refusal. A failure of TC-035 step 39 cannot say which part broke, and the matrix credits all of them or none. Fix: split into an observation/document AC, a settlement AC and a refusal AC, as AC-11/12/15-19 are split for the postcondition path. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:528 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-scope | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:113 | The state-clause path covers the `postcondition` state clause of one operation over one state object (the operation-contract harness of FR-015-AC-26), replayed as QSL's `Invocation` observation. State preconditions (`PreCall`) have no CG harness obligation of this kind and are not specified here. The object-type invariant (`Current`) is specified by FR-024-AC-57 and FR-024-AC-58 (IR-1505). |
| FR-024-AC-57 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:528 | PLANNED (IR-1505; prerequisite named by FR-015-AC-129). Prerequisites: FR-015-AC-38 (V2 invariant harness generation), the shared playback decoder of FR-024-AC-25 (IR-625) and QSL's `qsl_replay::replay_state_clause`. From a Falsified V2 invariant harness over one object type of one model, whose concrete playback binds every declared state field of that object, CG builds a `StateClauseCounterexample` whose observation is `ClauseSelectionInput::Current` (the current snapshot, the selected anchor and the self object) and not `Invocation`, ... |
| FR-024-AC-58 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:529 | PLANNED (IR-1505; prerequisite FR-024-AC-57; consumed by FR-015-AC-129). Over a `Bank` model with object `Account`, field `balance` typed `Int[0, 100]`, and the QSL invariant `invariant NonNegative using v on Bank::Account at current { self.balance >= 0 }`, two concrete playbacks replay through `replay_state_clause` as FR-024-AC-57 states and each settles `reproduced-with-evaluated-witness`, `violation` for the named clause: ... The unmutated invariant with playback `balance = 0` does not settle a violation. The test uses recorded playback text; installed-Kani verdicts for these mutants are FR |
| TC-035-step-39 | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:352 | 39. Decode the playback of a Falsified V2 invariant harness and build the Current-observation envelope (FR-024-AC-57). ... Mutation controls: building `Invocation` instead of `Current` fails the observation-form check in this step; a builder that ignores the playback values and always binds `0` fails on the `balance = -1` playback of step 40; a builder that selects the clause by position instead of its declared name fails when two invariants are declared in reverse order. |
| TC-035-step-40 | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:361 | 40. Over the `Bank`/`Account` fixture of FR-024-AC-58, replay `balance = 0` against the strengthened `balance >= 1` invariant, `balance = -1` against the widened-field package, and `balance = 0` against the unmutated invariant. The first two settle `reproduced-with-evaluated-witness`, `violation`; the third does not. A mutant that evaluates the unmutated text for the first playback fails the first assertion. |
| tests.md-TC-035-row | examined | spec/replay/matrix/tests.md:49 | / TC-035 / Verify counterexample submission in QSL's counterexample envelope / ... FR-024-AC-57, FR-024-AC-58 / 🚧 Planned (IR-1505 for AC-57/58) / |
| FR-015-AC-129 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:937 | ... Two invariant mutants each classify the invariant harness `Falsified` with a concrete in-domain counterexample from the FR-017-AC-63 witness phase, and replaying that counterexample through `qsl_replay::replay_state_clause` settles a reproduced violation of the same clause: the invariant text strengthened to `balance >= 1`, counterexample `balance = 0`; and the domain package's field widened to `Int[-1, 100]` with the invariant text unchanged, counterexample `balance = -1`. |

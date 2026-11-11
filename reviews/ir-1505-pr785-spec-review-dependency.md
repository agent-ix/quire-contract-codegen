---
id: SR-17940486709564847047674
title: "IR-1505 PR #785 Current-observation invariant replay: dependency analysis"
type: SpecReview
analysis: dependency
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

CHANGES REQUIRED (one medium). The prerequisites claimed not to block are verified. IR-625 is Done, and the shared decoder and parse_assertion_playback are on main. V2 invariant generation for a comparison-only scalar invariant is on main (FR-015-AC-49 PARTIAL, tagged). IR-917 / PR #583 covers only arithmetic clauses: invariant arithmetic is refused at v2.rs:184, and NonNegative and its `>= 1` mutant have no arithmetic, so IR-917 does not block. The pinned qsl-replay 434869e8 has every needed API (replay_state_clause, Current, by-name resolution, call_site ClauseName for invariants), so no QSL-side ticket is needed. The only open dependency question is the provenance of the 'recorded playback text'.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'The test uses recorded playback text' does not say where the text comes from. If it means a playback captured from a real Kani run of the mutant invariant harnesses, it depends on the FR-015-AC-129 installed run. That run measured MemoryExhausted at unwind 8, and IR-1436 orders the fit work, which contradicts the PR's 'no blocker' claim. If it means text hand-authored in Kani's concrete-playback format, the AC should say so. It should also require the text to be decoded through parse_assertion_playback and the shared decoder against the actually generated invariant harness (its identity and state_fields), so the test cannot bypass the decoder. Fix: state 'hand-authored in Kani's concrete-playback format for the harness CG generates for each mutant' and that no installed Kani run is required. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:529 |
| FND-002 | low | The prerequisite list cites FR-015-AC-38 as 'V2 invariant harness generation'. The invariant-specific criterion is FR-015-AC-49 (state and parameter domains via AC-39; invariant asserted, no subject call); AC-38 is the kind dispatch. Fix: cite FR-015-AC-38 and FR-015-AC-49. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:528 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-AC-57 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:528 | PLANNED (IR-1505; prerequisite named by FR-015-AC-129). Prerequisites: FR-015-AC-38 (V2 invariant harness generation), the shared playback decoder of FR-024-AC-25 (IR-625) and QSL's `qsl_replay::replay_state_clause`. From a Falsified V2 invariant harness over one object type of one model, whose concrete playback binds every declared state field of that object, CG builds a `StateClauseCounterexample` whose observation is `ClauseSelectionInput::Current` (the current snapshot, the selected anchor and the self object) and not `Invocation`, ... |
| FR-024-AC-58 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:529 | PLANNED (IR-1505; prerequisite FR-024-AC-57; consumed by FR-015-AC-129). Over a `Bank` model with object `Account`, field `balance` typed `Int[0, 100]`, and the QSL invariant `invariant NonNegative using v on Bank::Account at current { self.balance >= 0 }`, two concrete playbacks replay through `replay_state_clause` as FR-024-AC-57 states and each settles `reproduced-with-evaluated-witness`, `violation` for the named clause: ... The unmutated invariant with playback `balance = 0` does not settle a violation. The test uses recorded playback text; installed-Kani verdicts for these mutants are FR |
| FR-015-AC-129 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:937 | ... Two invariant mutants each classify the invariant harness `Falsified` with a concrete in-domain counterexample from the FR-017-AC-63 witness phase, and replaying that counterexample through `qsl_replay::replay_state_clause` settles a reproduced violation of the same clause: the invariant text strengthened to `balance >= 1`, counterexample `balance = 0`; and the domain package's field widened to `Int[-1, 100]` with the invariant text unchanged, counterexample `balance = -1`. |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at 551d1486. Verified in the fix commit: FR-024-AC-64: Each playback text is hand-authored in Kani's concrete-playback format for the invariant harness CG generates for that mutant, and is decoded through `parse_assertion_playback` and the shared decoder against that generated harness's identity and state fields; no installed Kani run is required, and the FR-015-AC-129 installed-backend run (IR-1436) is not a prerequisite. |
| FND-002 | fixed | Round 1 at 551d1486. Verified in the fix commit: FR-024-AC-57: Prerequisites: FR-015-AC-38 and FR-015-AC-49 (V2 invariant harness generation), ... |

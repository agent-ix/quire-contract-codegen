---
id: SR-1497
title: "Base checklist review of quire-contract-codegen PR 276 (IR-460 state-clause replay spec)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@d22cc5d849db3edd7bba68c3638bb9c4995ac723; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md (PR #276 diff vs origin/main 7345463; QSL qsl-replay locked at c8f0c28)"
review_set: subset
---

## Summary

Ticket: IR-460. Base checklist over the PR #276 diff: ID formats, AC numbering (FR-024-AC-11 to AC-17 contiguous after AC-10, no gap or duplicate), TC-035 steps 9-14 and expected results 9-14 trace to each new AC, the tests.md row and TC-035 Traces To list all seven, and every new AC is honestly PLANNED. Measured `quire coverage --strict`: main 301/422 backed, 56 unbacked; head 301/429 backed, 64 unbacked. The delta is exactly FR-024-AC-11..17 plus the new FR-024 matrix row, none counted as backed, so the PR's coverage claim holds. `quire validate` is clean on the four changed files. The measured QSL claims hold at c8f0c28: qsl-replay exports replay_state_clause, StateClauseCounterexample, StateClauseReplayResult, ClauseIdentityMismatch, StateClauseKind, ClauseSelectionInput, SnapshotValue, SelectedAnchor, ClauseSite, ClauseName, call_site and OperationSite, and has no invocation/snapshot document builder. `call_site` with a ClauseName returns ClauseSite{name, node, occurrence} and refuses an undeclared name with CallSiteRefusal::UnknownClause. No src file on origin/main names replay_state_clause or ClauseSite. Defects: the public entry and terminal settlement of the new path are not specified, and the real-Kani AC relies on a make target that runs only named test filters.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The PR adds a public state-clause replay path but no interface-001 entry (FrameReplay::new is listed for the frame path) and no FR-029/FR-030 reading of a StateClauseReplayResult, a state-clause call-site or replay refusal, the new typed Incomplete or the out-of-domain report into ReplaySettlement. TC-035 step 9 submits "through the generator" from an integration test, so the code PR needs a public function, and the terminal map of its results would be invented there. | FR-024-AC-11, FR-024-AC-14, FR-024-AC-16, interface-001, FR-029, FR-030 |
| FND-002 | low | FR-024-AC-17 / TC-035 step 14 say the real-Kani case runs only through `make kani`, but that target runs `--ignored` tests only for the filters kani_obligations, skeleton_spine, kani_witness_join, bounded_kani_corpus, kani_generation and kani_batching. A state-clause replay test whose name matches none of them is never run by any gate, so the AC could stay unbacked while reported as gated. | FR-024-AC-17, TC-035 step 14, Makefile:83-86 |

## Dispositions

Round 1, reviewed at 02a5f67616fed74c0c014d546e2938e555791968.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: interface-001 adds `StateClauseReplay::new`, with `replay` / `replay_through` and `StateClauseReplayError`, and run_terminal_value now names StateClauseReplayError's From impl. FR-029-AC-16 and TC-040 step 15 give the terminal reading. The FR-029 definition text was not extended to match; that is recorded as SR-1498 FND-005. |
| FND-002 | fixed | 02a5f67616fed74c0c014d546e2938e555791968: FR-024-AC-18 names `tc_035_real_kani_state_clause_counterexample_replays_through_qsl` in module `kani_obligations_state_clause_replay`. The Makefile kani target (`--test it ... -- --ignored ... kani_obligations ...`) selects it by substring, provided the module sits in the `it` test binary. |

Round 3, reviewed at 19196250716f86adf9af536a1cfce0399cd10424.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 19196250716f86adf9af536a1cfce0399cd10424: interface-001 now declares UnsupportedOperationShape{operation, declaration}, consistent with the FR-024 Behavior bullet ('carrying the operation and what it declares'), FR-024-AC-19 ('the operation and the declaration (its parameters and result)') and TC-035 step 17. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The interface-001 variant is `UnsupportedOperationShape{operation}`, but the FR-024 Behavior bullet ("naming the operation and what it declares") and FR-024-AC-19 ("naming the operation and the declaration") require the error to name the declaration too. The interface gives it no field for that. | interface-001 StateClauseReplay::new, FR-024-AC-19 |

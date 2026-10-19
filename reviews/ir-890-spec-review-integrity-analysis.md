---
id: SR-13287
title: "IR-890 CG PR 457 spec-review integrity: residue of the retired Input arm and the unavailable result"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@b8e2a28e8c2672090718bcb81d9617a060cf73dc vs origin/main e45b9d2f; spec/replay/functional/FR-016-witness-native-replay.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-026-witness-native-replay.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md"
review_set: subset
---

## Summary

Ticket: IR-890. Grepped `spec/`, `tests/` and `src/` at the PR head for `FR-024-AC-5`,
`FR-016-AC-12`, "unavailable", "five replay results", `Input` arm and corpus. No test or `src/`
tag cites either criterion. The FR-024 matrix row, TC-035 Traces To list and the spec/tests.md
Replay line drop FR-024-AC-5 consistently, and `make spec` / `quire validate` are clean for the
touched files. Three integrity defects remain: the PR removes the only source of an `Input`-arm
counterexample but keeps the criteria that reduce one, and two texts still rely on the removed
"unavailable" result.

## Examined criteria

- FR-024 Inputs/Outputs, FR-024:80-90, :121-122 (examined): a backend counterexample is now the only counterexample source; `ReplaySource` is `Witness` only.
- FR-024 statement, FR-024:327-328 (examined): "Where the generator derives reduced candidates from an `Input`-arm counterexample, it shall retain only `Input`-arm envelopes that preserve domain validity and the native verdict."
- FR-024-AC-9, FR-024:356 (examined): "A reduced `Input`-arm candidate is retained only when native replay preserves domain validity and the verdict, and it is itself an `Input`-arm envelope."
- FR-024-AC-40, FR-024:387 (examined).
- TC-035 step 7 and expected 7, TC-035:35, :125-126 (examined).
- FR-016-AC-4, FR-016:125 (examined): "A native replay that runs and disagrees with the harness result yields a typed mismatch, and never an unavailable result or a failure."
- FR-016:62-63, FR-016-AC-8 FR-016:129, TC-026:49 (examined): "none of the five replay results".
- spec/replay/matrix/tests.md:16, :40; spec/tests.md:20 (examined, clean).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The retirement is incomplete. The PR deletes the corpus counterexample from FR-024's Inputs and the `Input` arm from its Outputs, so no `Input`-arm counterexample can reach the generator, yet FR-024 still requires reducing one: the statement at :327-328, FR-024-AC-9, the `Input` clause of FR-024-AC-40, and TC-035 step 7 ("one `Input`-arm counterexample") with expected result 7 ("each retained `Input` envelope remains `Input`"). These are unsatisfiable against the PR's own Inputs. Fix in this PR: retire FR-024-AC-9 (ID not reused) and the `Input` sentence of AC-40, delete the :327-328 statement, reduce TC-035 step 7 and expected 7 to the `Witness` arm, and update the FR-024 matrix row, TC-035 Traces To and the spec/tests.md Replay line to match. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:327-328, :356, :387; spec/replay/matrix/TC-035-counterexample-envelope-intake.md:35, :125-126 |
| FND-002 | medium | FR-016-AC-4 still says a mismatch is "never an unavailable result". The PR removes the unavailable result from FR-016's Outputs and behaviour table, so AC-4 names an outcome that no longer exists. Fix: "yields a typed mismatch, and never a failure or a replay error". | spec/replay/functional/FR-016-witness-native-replay.md:125 |
| FND-003 | medium | "the five replay results" counted reproduced, malformed, out-of-domain, mismatch and unavailable. With unavailable gone there are four, so FR-016:63, FR-016-AC-8 and TC-026 expected results each name a count the spec no longer defines. Fix: replace with "none of the replay results" or list the four. | spec/replay/functional/FR-016-witness-native-replay.md:62-63, :129; spec/replay/matrix/TC-026-witness-native-replay.md:49 |

## Verdict

**CHANGES REQUESTED.** FND-001 is introduced by this PR's own Inputs edit and must be fixed here.

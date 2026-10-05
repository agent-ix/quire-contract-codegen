---
id: "SR-1510"
title: "CG PR 278 gap analysis: FR-024-AC-11 to AC-19, FR-029-AC-16, TC-035 steps 9 to 17, TC-040 step 15, interface-001 and AD-002 against StateClauseReplay"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@986f23cfa0bd371c73318ff285fbed44049af46d; spec/replay/functional/FR-024-counterexample-envelope-intake.md (Scope, Inputs, Behavior, AC-11 to AC-19, Current state), spec/replay/matrix/TC-035-counterexample-envelope-intake.md (steps 9 to 17, Status), spec/replay/matrix/tests.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md (AC-10, AC-16, Notes), spec/kani/matrix/TC-040-run-outcome-terminal-record.md (step 15, Status), spec/kani/matrix/tests.md, spec/core/functional/interface-001-codegen-api.md (StateClauseReplay::new, run_terminal_value), spec/assurance/AD-002-cg-qsl-replay-seam.md, src/replay/state_clause.rs, tests/it/kani_obligations_state_clause_replay.rs, tests/it/terminal_map.rs (git diff origin/main...HEAD, merge base e526390)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
---

# SR-1510: CG PR 278 gap analysis

## Summary

Ticket: IR-460. PR: agent-ix/quire-contract-codegen#278 at 986f23c. Plan completion: not assessed.
Semantic intent-to-test-to-code review was done by hand for the in-scope criteria; no subagent
fan-out was used.

Matrix: `quire coverage --scope <worktree> --format tsv` (quire 0.36.1) on `main` e526390 gives 66
unbacked rows and on 986f23c gives 44. I diffed the two sets. The 22 rows that drop are
FR-024-AC-1 to AC-19 (19 rows), the two FR-024 rows of `spec/replay/matrix/tests.md`, and the TC-035
row. AC-11 to AC-19 and the AC-11 to AC-19 matrix row (10 rows) are backed by tagged tests. The
other 12 (FR-024-AC-1 to AC-10, the AC-1 to AC-10 matrix row, and the TC-035 row) drop only because
TC-035 now has tagged tests, so the author's inflation claim is measured and correct. No planned
criterion is shown as covered in the documents. FR-024-AC-1 to AC-10 stay `Planned` in the matrix
row, FR-024's Current state ("meets none of FR-024-AC-1 to FR-024-AC-10 in full") and TC-035's
Status ("Steps 1 to 8 are not").

Per criterion (code and test read at 986f23c):

- AC-11: `replay` is `replay_through(replay_state_clause)`. The test compares it with a direct
  call for a violating run and a respecting run, checks the two differ, and swaps in a sentinel
  executor. Met. A faked verdict is caught.
- AC-12: the clause node and occurrence come from `call_site`'s `ClauseSite`, two clauses differ,
  and an unknown clause gives `CallSite(UnknownClause)`. Met.
- AC-13: the snapshot and invocation members match QSL's reader (SR-1509 Summary), and a changed
  value changes only its own snapshot. Met, with a weak oracle (SR-1509 FND-003). Gap
  FND-003 below: "the value the Kani playback bound" has no producer in `src`.
- AC-14: the vector test compares against literal RFC 8785 bytes, so a non-canonical encoder
  fails it, and a source/`Cargo.toml` inspection test. Met.
- AC-15: playback and post-state omissions per field. Met. A skipped check falls through to a
  QSL refusal and fails the `matches!`.
- AC-16: violating reproduces with `false`; respecting is `Inconclusive` / `Verdicts`. Met.
- AC-17: endpoints are admitted, one past each is refused. Met as written, but the test cannot
  tell the playback check from a post check (SR-1509 FND-002).
- AC-18: real-Kani test over a new subject; the merged AC text was edited to fit (FND-001). Not
  run by this review.
- AC-19: parameter and result fixtures are refused with the declaration. A skipped shape check
  surfaces `MissingField` and fails the test. The shape check runs after `admit` and
  `call_site`, not first as the PR body says, but before any document, which is what the AC
  requires. Met.
- FR-029-AC-16: `tc_040_the_state_clause_replay_reads_as_fr029_ac16` asserts real reproduced
  and inconclusive results, each CG variant mapping to `Failed`, and non-fault `Refused`,
  `CallSite` and `Dependencies` by code. A swapped settlement is caught. The fault clause is
  untested (FND-002).
- interface-001: the `StateClauseReplay::new` entry's variants equal the code's.
  `From<&StateClauseReplayResult> for ReplaySettlement` is not named in interface-001's
  `run_terminal_value` input list (which names `ReplayVerdict` for the function path). Not
  raised separately.
- AD-002: the seam table and the failure table updated; the post-state refusal path of FND-001 is
  not in AD-002's failure table.

## Verdict

FAIL. FR-024-AC-18 was amended inside a code PR to fit what the code does, and that hides a real
behavior gap (FND-001, high). FR-029-AC-16 is flipped to covered with an unbacked fault clause,
while FR-029-AC-10 stays planned for that same reason (FND-002). Every other in-scope criterion is
backed by a tagged test that would fail on the plausible mutant for it, apart from the oracle
weaknesses recorded in SR-1509.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-024-AC-18 (merged) required a debiting subject's real counterexample to settle `reproduced-with-evaluated-witness`. The canonical debiting subject's counterexample (balance 0, post -1) does not settle: QSL refuses the post snapshot, and FR-029-AC-16 reads that as `Inconclusive(ReplayRefused)`. The PR does not resolve or raise this. It adds `deposit_debiting_within_range` and rewrites the merged AC to "debits within the declared range", adding a sentence that blesses the refusal. That narrows a merged criterion to fit the code, inside a code PR. It leaves a real case unspecified: the subject leaves the model's range, or the harness does not bound the post state. That case is now misreported as a QSL data refusal, against FR-029-AC-16's intent that CG-side value defects never read as `Inconclusive(ReplayRefused)`. Restore the merged AC-18 text. Raise the post-state case for an owner or spec decision: bound the post state in the harness, add a typed CG error, or rule it a refutation. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:219, src/replay/state_clause.rs:332-333 |
| FND-002 | medium | FR-029-AC-16 is flipped to Covered, but its text includes "fault `Failed`" for `Refused` and `CallSite`. No test builds that fault, for the same reason FR-029-AC-10 stays Planned (QSL's `InternalFault` is unreachable from `qsl-replay`). The PR's own Notes and TC-040 Status say this. So one criterion with an untested clause is Covered while its sibling is Planned for the identical reason, and the strict count treats AC-16 as backed. Mark AC-16 partly covered (as FR-029-AC-3 is handled), or move the fault clause under AC-10 | spec/kani/functional/FR-029-run-outcome-terminal-record.md:181, spec/kani/matrix/tests.md:38 |
| FND-003 | medium | No `src` producer for the state-clause playback. FR-024 Inputs say the playback is "decoded by FR-016 into values named by their argument bindings", and AC-13 and AC-18 speak of "the value the Kani playback bound". `StateClauseReplayInputs::playback` takes caller-named `i64` values. The only decoder is test-only `playback_state`, which reads the harness's two fields by position. The PR records this in Current state but no criterion or ticket owns it, so the pipeline cannot be driven from a Kani run in `src` | src/replay/state_clause.rs:243-244, tests/it/kani_obligations_state_frame.rs:1026, spec/replay/functional/FR-024-counterexample-envelope-intake.md:240-242 |
| FND-004 | low | The strict-coverage drop (66 to 44) counts 12 rows as backed that have no test of their own: FR-024-AC-1 to AC-10, their matrix row, and the TC-035 row. They are backed only because they share TC-035 with the new tests. The documents stay truthful, but any gate reading the engine's unbacked count now misses ten planned criteria. Split TC-035, or record this in tests.md beside the AC-1 to AC-10 row | spec/replay/matrix/tests.md:16-17 |

## Dispositions

Reviewed at 33d2f1205611b644ff2c41a5c1307bbcbe02159e. Strict coverage re-measured: `main`
bcce798 has 66 unbacked rows and the head has 44. The same 22 rows drop, 10 of them honestly.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed a86a610 | FR-024-AC-18 is byte-identical to `main`'s row apart from the dropped `PLANNED (IR-460).`, compared with `diff`. No criterion text approves the QSL refusal. The in-range subject's reason lives in the AC-18 test doc comment, and FR-024 Current state records the post-state question as open. The code-side behavior stays deferred under SR-1509 FND-001 |
| FND-002 | fixed a86a610 | FR-029-AC-16 now keeps only the non-fault reading of `Refused` and `CallSite`. Their fault readings are added to FR-029-AC-10, which stays Planned, and the Notes, TC-040 Status and kani tests.md rows agree |
| FND-003 | deferred | FR-024 Current state records the missing state-playback decoder as "Unbuilt, with no owner yet". It is out of scope for this PR's criteria, and it needs a ticket to own it |
| FND-004 | accepted-no-change | The artifact is stated in FR-024 Current state and in the tests.md AC-1 to AC-10 row ("no test of its own"). The criteria stay Planned. It is a property of quire's AC-to-TC walk, not of this PR |

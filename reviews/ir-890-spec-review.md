---
id: SR-13285
title: "IR-890 CG PR 457 spec-review: FR-016-AC-12 fault reading and FR-024-AC-5 retirement"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@b8e2a28e8c2672090718bcb81d9617a060cf73dc vs origin/main e45b9d2f; spec/replay/functional/FR-016-witness-native-replay.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-026-witness-native-replay.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md"
review_set: subset
---

## Summary

Ticket: IR-890. Spec-only PR (6 Markdown files under `spec/`). No production code in the diff, so
code-review and rust-review do not apply. Sub-analyses: EARS (SR-13286), integrity (SR-13287);
gap-analysis is SR-13288.

Claims re-measured on origin/main e45b9d2f:

- `src/replay/function.rs:100` declares `SpineReplayError::Refused(Box<ReplayRefusal>)`, and
  `:270` maps every `Err(refusal)` from `replay` to it, `Fault` included. Confirmed.
- `src/kani/terminal.rs:34-37` documents `ReplaySettlement::Refused` as read by
  `TerminalValue::from_replay_refusal` (fault gives `Failed`); `:234` routes it there; `:238`
  maps `ReplaySettlement::Fault | CgDefect` to `Failed`. `src/replay/function.rs:962` converts
  `SpineReplayError::Refused` to `ReplaySettlement::Refused`. Confirmed.
- FR-029-AC-10 (spec/kani/functional/FR-029-run-outcome-terminal-record.md:370) and
  `tc_040_a_fault_in_any_replay_wrapper_is_failed` (tests/it/terminal_map.rs:535-543, tagged
  `FR-029-AC-10`) assert `Failed` for `ReplayRefusal::Fault` wrapped in `SpineReplayError::Refused`.
  Confirmed.
- FR-024-AC-5 retirement: the row is removed, the ID is recorded as retired and not reused
  (FR-024:409, :418), no test or `src/` tag cites it (grep of `src/`, `tests/`), and `quoin matrix`
  no longer lists it. The justification (no corpus counterexample packet; corpus path removed under
  FR-015-AC-109..111) agrees with FR-015 and with
  `src/kani/generate/corpus/bounded_kani_corpus.rs:927`. Confirmed.
- No hash, pin or compatibility layer is added. `make spec` passes; its warnings are pre-existing
  in FR-014 and FR-017, which the PR does not touch.

The retirement is justified but incomplete (SR-13287 FND-001), and the old "unavailable" meaning
survives in FR-016-AC-4 and the "five replay results" count (SR-13287 FND-002, FND-003).

## Examined criteria

- FR-016-AC-12, spec/replay/functional/FR-016-witness-native-replay.md:133 (examined): "A `replay` call that returns `ReplayRefusal::Fault` retains that fault as `SpineReplayError::Refused`; the run-outcome terminal value is `Failed` under FR-029-AC-10."
- FR-016 Outputs, FR-016:35-41 (examined): "A typed replay result (reproduced failure or evidence failure), or a typed replay error. A `ReplayRefusal::Fault` remains in the replay error and maps to terminal `Failed` under FR-029-AC-10."
- FR-016 behavior row for Fault, FR-016:84 (examined).
- FR-024 Inputs/Outputs edits, FR-024:80-90, :121-122 (examined).
- FR-024 current state, FR-024:404-418 (examined).
- TC-026 procedure/expected/status edits, TC-026:24-30, :50-56, :112-116, :131-135 (examined).
- TC-035 steps 5 and expected 5, status, TC-035:14-17, :32, :116, :229-232 (examined).
- spec/replay/matrix/tests.md:16, :40 and spec/tests.md:20 (examined).
- FR-029-AC-10, spec/kani/functional/FR-029-run-outcome-terminal-record.md:370 (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-016-AC-12 is compound and duplicates another criterion: its second clause ("the run-outcome terminal value is `Failed` under FR-029-AC-10") restates FR-029-AC-10, which TC-040 already backs, while AC-12 is verified by TC-026, and TC-026's procedure now asks it to inspect the terminal value too. Two criteria in two TCs then own one assertion. Fix: make AC-12 assert only the replay-layer fact (a `Fault` from `replay` is returned as `SpineReplayError::Refused` carrying that fault, never a mismatch or an evidence-failure verdict) and name FR-029-AC-10 as the owner of the terminal reading in prose; drop "and run-outcome terminal value" from TC-026:29. | spec/replay/functional/FR-016-witness-native-replay.md:133, spec/replay/matrix/TC-026-witness-native-replay.md:29-30 |

## Verdict

**CHANGES REQUESTED.** The direction is right and every code claim re-measures true. Fix FND-001
here and the integrity findings in SR-13287 (one high) before merge.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | 835d61d9: FR-016-AC-12 now reads "A `replay` call that returns `ReplayRefusal::Fault` is returned as `SpineReplayError::Refused` carrying that fault."; TC-026:29 now "Inspect the replay error for the fault"; TC-026 expected results say FR-029-AC-10 owns the terminal `Failed` reading. |

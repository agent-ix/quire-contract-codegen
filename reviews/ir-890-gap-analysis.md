---
id: SR-13288
title: "IR-890 CG PR 457 gap-analysis: FR-016-AC-12 and FR-024-AC-5 against code and matrix"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@b8e2a28e8c2672090718bcb81d9617a060cf73dc vs origin/main e45b9d2f; spec/replay/**, src/replay/function.rs, src/kani/terminal.rs, tests/it/terminal_map.rs"
review_set: subset
---

## Summary

Ticket: IR-890. Plan completion: not assessed (planless). Read `quoin matrix --json` at the PR head
and the code paths the PR cites.

- FR-016-AC-12: `untagged`, "no evidence is bound". The behaviour it now states is implemented:
  `src/replay/function.rs:270` returns `SpineReplayError::Refused` for every `ReplayRefusal`.
- FR-029-AC-10: bound to `tests/it/terminal_map.rs:537` `tc_040_a_fault_in_any_replay_wrapper_is_failed`.
- FR-024-AC-5: absent from the matrix after the PR; no binder anywhere.
- No code is orphaned by the change: `ReplaySource::Input` uses in `src/` are the input-arm
  evaluation path (`src/replay/function.rs:610`) and composite builder, not a corpus intake.

## Examined criteria

- FR-016-AC-12, spec/replay/functional/FR-016-witness-native-replay.md:133 (examined).
- FR-029-AC-10, spec/kani/functional/FR-029-run-outcome-terminal-record.md:370 (examined).
- src/replay/function.rs:100, :270, :962 (examined).
- src/kani/terminal.rs:34-43, :234, :238 (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-016-AC-12 stays Planned with no test although its replay-layer behaviour is implemented and cheap to back: QSL's `InternalFault` is constructible (tests/it/terminal_map.rs:494) and `replay_counterexample_through` (src/replay/function.rs:1098) and the private `replay_falsification_through` (:210) take an injected executor. Fix (in this PR or a follow-up ticket): add a `#[trace("FR-016-AC-12", "TC-026")]` test that injects `Err(ReplayRefusal::Fault(..))` and asserts `SpineReplayError::Refused` carrying the fault, then mark the row Covered. | spec/replay/functional/FR-016-witness-native-replay.md:133, src/replay/function.rs:270 |

## Verdict

**PASS WITH NITS.** Code claims re-measure true; one low coverage gap.

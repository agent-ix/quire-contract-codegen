---
id: "SR-634"
title: "IR-92 slice 2 gap analysis: decode path cut-over and remaining IR witness uses"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@92ec95afe85b8af23e3123105d9e7120a5b452ff; src/kani_witness_join.rs, src/spine_replay.rs, src/bounded_kani_replay.rs, src/bounded_kani_corpus.rs, tests/it/bounded_kani_corpus.rs, spec/functional/complete-v1/FR-016-witness-native-replay.md, spec/test-matrix.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: reviews
---

# SR-634: IR-92 slice 2 gap analysis

## Summary

Ticket: IR-92 (slice 2). PR: agent-ix/quire-contract-codegen#204, head 92ec95a. Method:
gap-analysis, planless. It is scoped to the PR's diff and to the IR witness uses the PR says remain.
Plan completion: not assessed.

## Method

I checked FR-016-AC-1, AC-5 and AC-8 against the decoder and its tests. I grepped `src/` and
`tests/` for every remaining use of `quire_contract_ir::kani::{Witness, WitnessValue,
WitnessValueType, WitnessBinding, ReplaySource, ReplayAgreement, CounterexamplePacket}`, and
compared the result with the PR body's list. I ran the ignored real-Kani TC-026 lane myself (see
SR-633).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-016-AC-1 ("every retained witness decodes into typed values or is refused as malformed") is violated for real backend witnesses. A well-formed Kani contract-harness transcript is refused as malformed (`kani_witness_check_text_missing`; SR-633 FND-001). The only tests that decode a real transcript are in the ignored Kani lane, and both fail at this head. Yet test-matrix.md:51 marks AC-1 and AC-8 Covered, and TC-026's Status says the real-backend decode runs in that lane. | src/kani_witness_join.rs:266-270, spec/test-matrix.md:51 |
| FND-002 | low | The rest of the cut-over has no tracker ticket. IR-347 is blocked on CG leaving IR's replay types, and this PR leaves the corpus and native Input-arm replay on them. IR-92's children and relations name no ticket for that work. | src/bounded_kani_replay.rs:3-4, src/bounded_kani_corpus.rs:11-13 |

## Verdict

Not mergeable until FND-001 (SR-633 FND-001) is fixed and the Kani lane is green. FND-002
needs a ticket before IR-347 can proceed.

The PR's list of leftover IR witness uses is accurate and complete. Each use really remains, and
there are no others:
- `src/bounded_kani_replay.rs`: `replay_counterexample`, `CounterexamplePacket` and
  `ReplayAgreement` at 3-4, 12 and 14; the test imports of `ReplaySource` and `WitnessValue` at
  22-24; `ReplaySource::Input` and `WitnessValue::Integer` at 53-55; `ReplayAgreement::Input` at 67.
- `src/bounded_kani_corpus.rs`: the imports at 11-13; `CounterexamplePacket` at 172 and 328;
  `ReplaySource::Input` at 335; the `WitnessValue` conversion at 480-498; the test at 878-880.
- `tests/it/bounded_kani_corpus.rs`: `ReplayAgreement::Input` at 220 and 441;
  `WitnessValue::Integer` at 232, 236, 452 and 456.

The decode and QSL replay path (`kani_witness_join`, `spine_replay`) no longer touches any IR
witness type.

Stopping at this partial cut-over is acceptable as a slice. The decode path is one coherent
unit, from a Kani transcript through a qsl-replay `WitnessValue` to `qsl_replay::replay`. The
corpus path is a different mechanism: an IR `execute_native` Input-arm replay of cases that
carry no QSL source package, which `qsl_replay::replay` requires. Moving it is a design step of
its own, not unfinished work of this one. What makes the slice acceptable is that the remainder
is tracked (FND-002) and this slice's own path works on real transcripts (FND-001).

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6feece1: a real Kani contract transcript with a multi-line check text now decodes. Both ignored real-Kani TC-026 tests pass (reviewer run), so FR-016-AC-1 and AC-8 hold for real witnesses. |
| FND-002 | deferred | The remaining corpus and native Input-arm replay is retired in CG PR #205 (open, head 75f4303). IR-453 (Backlog) tracks backing the corpus rows honestly through qsl_replay. |

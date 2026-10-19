---
id: SR-13303
title: "IR-891 PR 462 code review (with Rust lane): FR-016-AC-6 required witness decode bounds"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@a18d5a991b34b9e2a74c8ea7cc6dbf3d436ad907 vs merge base 58f44780; src/replay/{witness,function,frame,state_clause}.rs, src/kani/output/playback.rs, src/lib.rs, tests/it/{bounded_kani_corpus,composite_original_eq,kani_obligations_state_clause_replay,kani_obligations_state_frame,kani_witness_join,skeleton_spine,terminal_map}.rs, tests/state_frame_support/native_twin.rs (diff origin/main...HEAD)"
---

# SR-13303: IR-891 PR 462 code review

## Summary

Ticket: IR-891 (FR-016-AC-6). PR agent-ix/quire-contract-codegen#462 at a18d5a99, six commits on
merge base 58f44780. The Rust lane is folded into this file.

The PR adds `WitnessDecodeLimits { bytes, values }` (derives `Clone, Copy, Debug, Eq, PartialEq`,
no `Default`). It is a required parameter of the public `decode_falsification`, of
`StateClauseReplayInputs::decode_falsification` and of `BoundedCorpusCase::replay_counterexample`,
and a required field `ReplayInputs::decode_limits`. The internal `decode_playback` checks
`transcript.len() > limits.bytes` before selecting a playback block. `concrete_entries` refuses
before parsing the value past `limits.values`. Function, harness, frame, state-clause and corpus
paths all pass the limits on. The state-clause decoder uses the per-dimension minimum of its
parameter and `run.decode_limits`.

What the reviewer measured in a fresh worktree with its own target dir:

- Every call site of `decode_playback`, `read_block` and `concrete_entries` receives caller limits.
  `counterexample_playback` and `playback_blocks` select the payload during the Kani run and do
  not decode values, so they are out of AC-6 scope. The five `usize::MAX` limits in
  `src/replay/function.rs` are all inside `#[cfg(test)]` helpers. No `Default`, no hidden constant
  and no compatibility layer is added.
- Trace tags are `#[trace("TC-026", "FR-016-AC-6")]` only, with no `Trace:` doc lines.
- Focused tests pass: the witness unit test, plus the function, frame and state-clause poisoned
  suffix tests. Kani spot-check (serial, under the lane lock): `tc_026_corpus_replay_refuses_poisoned_suffixes_at_both_decode_bounds`
  and `tc_023_real_corpus_playback_replays_through_its_qsl_source` pass. The full 36-case lane was
  not rerun.
- `make fmt-check`, `make lint` and `make spec` pass.
- Downstream consumers (workspace grep outside this repo and its worktrees): the direct callers are
  `quire-integration-ir-kani` and `quire-integration-ir699` `tests/qsl_kani_exemplar.rs`
  (`ReplayInputs {`, `decode_falsification(`), as the PR body lists. `quire-driver` uses none of
  the changed signatures. `quire-spec-language/integration/current-head` depends on this crate but
  uses only `BOUND_COVERAGE_SCHEMA`. The list is complete.

Reviewer production mutants, each reverted after its run. All of them were killed:

- byte comparison `>` to `>=`; byte check removed; value comparison `>=` to `>`; value check
  removed;
- `replay_counterexample_through` decodes with unbounded limits;
- `replay_harness_counterexample_through` decodes with unbounded limits (killed only by the Kani
  corpus test);
- `FrameReplay::new` decodes with unbounded limits;
- the state-clause decoder ignores its parameter, or ignores `run.decode_limits`;
- the corpus package ignores the caller's limits.

The PR's mutation claims hold. The surviving `Default` mutant is recorded in SR-13304.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The two new refusals fill `DecodeFailure.context` differently. `cg_witness_decode_bytes_limit` puts the byte bound in `context`, which replaces the module symbol that every other decode code carries there. `cg_witness_decode_values_limit` keeps the module symbol and does not report the bound. A consumer cannot read the bound that was exceeded the same way for both. Fix: use one convention for both codes, for example the module symbol as context and the bound in the code's documented context, or the bound for both. | src/replay/witness.rs:165-171, src/kani/output/playback.rs:276-278 |
| FND-002 | low | `WitnessDecodeLimits { bytes: usize::MAX, values: usize::MAX }` is written out 42 times in the diff. This includes five `#[cfg(test)]` helpers in function.rs and two test-support modules. The signatures in frame.rs and state_clause.rs use the full `crate::replay::witness::WitnessDecodeLimits` path, although both modules already import from `crate::replay::witness`. Fix: add one test-only `unbounded()` constructor or helper, and import the type. | src/replay/function.rs:733-829, src/replay/frame.rs:343, src/replay/state_clause.rs:457-462, tests/it/kani_obligations_state_clause_replay.rs:191-380 |

## Verdict

**PASS on correctness, with two low findings.** Both bounds are enforced in the shared reader,
before any block selection and before any value past the bound is parsed. Every decode path takes
caller limits with no default. The at-bound and poisoned-suffix controls are real: each production
mutant above makes a test fail, and each poisoned suffix produces a distinct error when it is read.
The low findings are consistency and tidiness only. Whether the PR can merge depends on the medium
findings in SR-13304.

## Dispositions

Round 1, against fix commit 61a15bc2 (PR head 61a15bc22a1394afc04dfc936971bf2855a3f74d). The reviewer checked it in a fresh worktree: the focused tests pass, and `make fmt-check`, `make lint` and `make spec` pass.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Both limit refusals now carry the module symbol with `;bytes=N` or `;values=N` appended (witness.rs:174-178, playback.rs:278-282). The `DecodeFailure.context` doc says so, and the unit test asserts both contexts. |
| FND-002 | fixed | Test-only `unbounded_decode_limits()` helpers are added in witness.rs (`#[cfg(test)]`), tests/it/main.rs and native_twin.rs, and they replace the repeated literals. frame.rs and state_clause.rs now import `WitnessDecodeLimits`. |

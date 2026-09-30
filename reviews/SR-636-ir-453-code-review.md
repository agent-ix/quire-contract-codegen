---
id: "SR-636"
title: "CG PR 205 code review: retire the bounded-Kani corpus native replay"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@d7a865dc067b1d002c455262eba60b63972b61d9; src/bounded_kani_corpus.rs, src/bounded_kani_replay.rs (deleted), src/lib.rs, tests/it/bounded_kani_corpus.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-023
    type: references
---

# SR-636: CG PR 205 code review

## Summary

Ticket: IR-453 (the later honest replacement this retirement hands off to; the PR branch
`chore/retire-corpus-native-replay` names no ticket). PR: agent-ix/quire-contract-codegen#205,
head d7a865d, base main bb8523f. This review covers code-review with the rust-review lane folded
in, scoped to `git diff origin/main...HEAD`.

## Method

I read the whole diff and the surrounding code at the head. I grepped `origin/main` for every
caller of each removed symbol (`replay_codegen_counterexample`, `BoundedCorpusCase::counterexample`,
`arithmetic_assignments`, `graph_assignments`, `collection_assignments`, `build_assignments`,
`integer_assignment`, `kani_corpus_assignment_out_of_range`). I grepped every sibling checkout
under `~/dev` for external users of the removed public API, including the one downstream crate
that depends on CG (`quire-spec-language/integration/current-head`). I listed the remaining
Contract IR witness and replay users at the head. I compared the deleted tests with the tests
that remain, and ran a mutation probe on the graph arm. I test-merged the head with PR #204's
head (92ec95a). I ran `make ci` at the head with a private scratchpad TRUSTED_HOME.

Premise, measured on main bb8523f: both integration callers pass a closure that ignores its input
and returns `KaniOutcome::counterexample` (tests/it/bounded_kani_corpus.rs:213 and :434). The
unit test in `src/bounded_kani_replay.rs` has two legs. Line 63 is the same always-counterexample
closure. Line 71 is an always-`proved` closure that checks Contract IR's disagreement refusal. That
is IR-owned behaviour, and IR covers it in its own tests (quire-contract-ir
`tests/it/kani_replay.rs:53` on IR main). No closure evaluates the input. The ruling holds,
with one correction: not every closure returns a counterexample, but every closure is constant.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The deleted unit test `tc_023_false_case_retains_a_replayable_counterexample_packet` was the only test that asserts a graph-family case classifies as false: an unreachable `b -> a` request gives `boolean_claim() == Some(false)`. That assertion is independent of the packet. It is real behaviour, because IR's `prepare_finite_graph_reaches` computes reachability. The PR deletes it together with the packet assertions, although it kept the matching `Some(false)` assertion in the collection integration test. Every graph request left in the corpus tests is the reachable `a -> b`. So replacing `lowered.reachable` with `true` on the graph arm no longer fails any test, and TC-023 requires a counterexample case for each semantic family. The test should keep its generation step and its `Some(false)` assertion, and drop only the packet lines. | src/bounded_kani_corpus.rs:262-265, src/bounded_kani_corpus.rs:745 |

## Verdict

Mergeable after FND-001 is fixed. The fix restores about 20 test lines. Nothing else blocks.

What is right:
- Nothing still needed was deleted. Each removed symbol's callers are inside the deleted file,
  the corpus's own packet construction, or the tests that replay through the constant closure.
  No sibling checkout and no downstream crate calls `replay_codegen_counterexample` or reads
  `BoundedCorpusCase::counterexample`. The only other mention is prose in QSL
  ADR-010:705 and OBS-028, which is historical. `kani_corpus_assignment_out_of_range` appears in no
  spec, schema or other test. Case generation and classification are unchanged. The case counter
  is still taken after every fallible step. The two `consumes_no_case_number` dependency tests
  (src/bounded_kani_corpus.rs:652 and :691) still cover that invariant.
- The renamed `tc_023_kani_falsifies_the_generated_false_collection_harness`
  (tests/it/bounded_kani_corpus.rs:317) still requires
  `classify_kani_run(..) == KaniRunOutcome::Falsified { .. }` and a nonzero exit. A nonzero exit
  alone does not pass it.
- The public API matches the spec. `src/lib.rs` drops the `mod` and the `pub use`, and
  interface-001 drops the operation and its feature-table row. No other export changes.
- IR `kani/replay.rs` users left in CG: none. After this PR there is no use of IR
  `replay_counterexample`, `CounterexamplePacket`, `ReplaySource`, `ReplayAgreement` or
  `FiniteInput`-as-packet. The IR witness users that remain are:
  `src/kani_witness_join.rs:35-37` (IR `Witness`, `WitnessBinding`, `WitnessValue`) and :98-99
  (`WitnessValueType`); the doc comment at `src/kani_transcript.rs:11`; `src/spine_replay.rs:18`
  (IR `WitnessValue`; its `Witness` and `ReplaySource` are `qsl_replay`'s);
  `tests/it/skeleton_spine.rs:32` and `tests/it/kani_witness_join.rs:34` (IR `WitnessValue`).
  PR #204 changes all of these files, so the PR body is accurate.
- PR #204 interplay: `git merge-tree` of d7a865d with 92ec95a is clean. #204's `lib.rs` and
  interface-001 hunks touch other exports and operations. Neither PR references a symbol the
  other removes.
- Rust idioms: this is a deletion. It leaves no dead imports (clippy `-D warnings` is clean), no
  new `unwrap`, `allow` or `pub` surface, and no CI workflow change.

Mutation probe at d7a865d: I replaced `lowered.reachable` with `true` on the graph arm
(src/bounded_kani_corpus.rs:264) and ran `cargo test --locked bounded_kani_corpus`. All 10 unit
tests and all 6 integration tests passed, so the mutant survives. The deleted test asserted
exactly the `Some(false)` that this mutant flips. The worktree was restored afterwards.

Gates at d7a865d: `make ci` with a private scratchpad TRUSTED_HOME exited 0. That run covered fmt-check,
spec (`quire validate`; only the existing FR-014 EARS warnings), clippy `-D warnings`, the MSRV
1.98.1 tests (77 unit and 221 integration passed, 5 ignored), deny (advisories, bans, licenses and
sources all ok), audit-unsafe, rustdoc, and test (77 and 221 passed, 5 ignored). The three real-Kani
corpus tests ran in the default lane and passed, including
`tc_023_kani_falsifies_the_generated_false_collection_harness`.

---
id: SR-17940486709564847047624
title: "IR-1022 PR #776 two-phase Kani run: code review"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #776 @ b09ac3dc vs origin/main; IR-1022; src/kani/abi.rs, src/kani/classify.rs, src/kani/identity.rs, src/kani/identity/constrained.rs, src/kani/generate/**, src/kani/run/execute.rs, src/kani/run/execute/batch_tests/two_phase.rs, src/kani/run/harness.rs, src/kani/run/scalar.rs, src/kani/terminal.rs, src/kani/test_support.rs, src/routed/generate.rs, tests/it/*.rs (diff only)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
---

## Summary

Ticket: IR-1022. Code review with rust-review lane of PR #776, head b09ac3dc2c62eb8986ec5390d044a5c1a84303f4, one commit on origin/main, 25 files, +1382/-83. The PR implements the two-phase Kani run of FR-017-AC-63, decision D1 of IR-1436. The identity now records a proof vector, with no concrete playback, and a witness vector. The proof vector launches first. The witness vector launches only after a failed non-unwinding check. `conclude_witness` compares the failed (class, location) sets. `WitnessDisagreed` is a new reason. `WitnessPhaseRefused` carries the proof evidence. Batch members held after the proof phase are regrouped by witness vector and ceilings. Evidence gains a nested `proof_phase`.

Reviewer runs, all on Linux, no Kani or CBMC, no `make ci`:

- **Baseline.** `cargo test --lib --locked -- kani::run kani::terminal kani::generate::spec kani::abi kani::classify` gave 168 passed and 0 failed. All 15 `two_phase` tests passed.
- **Mutant run.** One combined mutant run of `batch_tests::two_phase`. Patch kept at scratchpad `ir1022-review-mutants.patch` and reverted afterwards. Results:
  - (M1) Compare only location in `conclude_witness`: caught by the `other-class` case of `a_witness_that_does_not_reproduce_the_failed_checks_disagrees` (Falsified instead of WitnessDisagreed).
  - (M2) Raise the witness launch's wall clock 1.5 times: caught by `a_witness_phase_is_stopped_at_the_unchanged_wall_clock_ceiling` (WitnessDisagreed instead of TimedOut).
  - (M3) Raise the witness launch's memory ceiling 1.5 times: **survived**. `a_witness_phase_is_stopped_at_the_unchanged_memory_ceiling` passed.

## Verdict

CHANGES REQUIRED. What is right:

- **Vectors.** The proof and witness vectors are as FR-015-AC-9 states. The witness vector is byte-equal to the pre-IR-1022 single vector, with or without stubbing.
- **Witness gate.** The witness launches only for Falsified or FailedWithoutCounterexample proof outcomes that have a failed non-unwinding check. An unwinding failure classifies on its own.
- **Falsified.** Falsified requires the witness's own Falsified, which needs a printed playback, plus set equality of (class, location) in both directions. Every other success or unwinding witness is WitnessDisagreed.
- **Witness stops.** Witness TimedOut, MemoryExhausted, NoVerdict and Cancelled keep their reasons with no third launch.
- **Refusals.** A refused witness launch or report is wrapped with the proof evidence.
- **Batches.** Proof batches split held members out. Witness groups regroup by witness vector and ceilings, and proofs are re-paired in member order.
- **Terminal mapping.** WitnessDisagreed maps to Failed and EnvironmentFault.

**Installed-Kani measurement.** The coder's measurement (healthy and exact-999 mutant both Inconclusive TimedOut at about 600 s, 13.2/13.4 GB) was reported honestly on the ticket. The PR body lists it under "Not verified". Keeping the measurement test out of the PR is consistent with AD-007, which says the known-bad healthy-clause test is not run or gated on. The ticket's Verified and in-domain-playback acceptance is therefore unmet, and this PR must not close IR-1022.

**Defects:**

- The memory-ceiling mutant control of AC-63(c) cannot fail.
- The recorded-ceiling assertions are tautological.
- A wrapped witness refusal loses its class and code.
- The non-Linux report fixture is single-phase while its tests now expect a witness.
- Public docs are stale.
- PR-body wording is inaccurate.
- Merge-time auto-close risk.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-017-AC-63(c) memory-ceiling mutant control cannot fail. The "one-and-a-quarter" memory witness stand-in allocates a 40 MiB bytearray in python3 under a 32 MiB ceiling. Interpreter and shell RSS put the tree near 1.6 times M, not the 1.5 times M the AC requires. A reviewer mutant raising only the witness launch's memory ceiling 1.5 times (M3) survived: the test still sees MemoryExhausted. The PR body's claim that a raised memory ceiling is caught does not hold at the AC's 1.5 factor. The wall-clock twin, sleep 2.5 s against a 2 s ceiling, did catch the 1.5 times raise. Fix: size the stand-in so the whole tree stays below 1.5 times M and above M, then re-run the 1.5 times raise mutant. | src/kani/run/execute/batch_tests/two_phase.rs:347-382 |
| FND-002 | medium | The recorded-ceiling assertions in two_phase (a)/(c) cannot detect a raised witness ceiling. They are `evidence.ceilings == identity.ceilings` and `proof_phase.ceilings == evidence.ceilings`. `evidence_of` copies `harness.ceilings` from the identity, not the ceilings `start()` applied. AC-63(c) says a raising mutant yields "a recorded ceiling larger than the proof launch's, which fails the case", but no launch-only raise can change the recorded value. Detection rests entirely on the stand-ins, and FND-001 shows the memory one is too loose. Fix: record the ceilings actually passed to the bounded launcher, or drop the recorded-ceiling leg from the AC/test claims. | src/kani/run/execute.rs:556-576 |
| FND-003 | medium | `WitnessPhaseRefused` flattens the nested refusal. `KaniExecutionRefusal::code()` returns None for it, so a witness-phase OutputOverLimit or OutputUnread loses its stable `kani_output_over_limit` / `kani_output_unread` code. `KaniScalarNativeError::class()` maps it to EnvironmentFault even when the inner refusal is a CompletionRefusal kind (OutputOverLimit, PlaybackForNonMember). The witness phase is the only phase that prints playback, so an output-cap refusal there is the likely case. FR-017-AC-42's "every other nested refusal" wording makes this spec-literal, but the code and class of the inner refusal are silently lost. Fix: delegate `code()` and the class to the inner refusal, or amend AC-42 to state the flattening deliberately. | src/kani/run/execute.rs:175-195 |
| FND-004 | low | The non-Linux `report_fixture::single`/`groups` still runs the proof phase only (`Phase::Proof`, no witness). The un-cfg'd report-fixture tests rewritten to `StandIn::replaying_phases` now expect the witness-phase playback, so they would fail on a non-Linux target. Those tests are `tc_043_members_are_matched_by_path_and_classified_from_their_own_entry` and the exit-explained, timeout-entry, own-playback and two-failed-checks cases. The coder states the non-Linux fixture was not built. CI is ubuntu-only, so this is latent. | src/kani/run/execute.rs:1486-1525 |
| FND-005 | low | Public docs are stale for two-phase. The module doc, `execute_kani_obligation` and `execute_kani_obligations` do not mention the witness phase. `execute_kani_obligations` still says "each process has its own result, in order of its first member", but witness group runs are appended after the proof groups. `KaniGroupRun` still says "What one launcher process (one group) of a batch yielded", but a witness group's run now spans the proof process and the witness process, and a proof group's run may omit held members. | src/kani/run/execute.rs:875-912 |
| FND-006 | low | The PR body says the proof vector "keeps `--slice-formula`". Neither vector contains a `--slice-formula` option: formula slicing is the backend default that concrete playback disables. The abi.rs doc comment states this correctly. The body also reports only the healthy proof's TimedOut and omits that the exact-999 mutant also timed out at about 600 s (13.4 GB), which the ticket comment does record. | PR #776 body |
| FND-007 | medium | Merging can close IR-1022 with its acceptance unmet. The PR title is "IR-1022: ..." and the branch is `code/ir-1022-two-phase-run`. Linear's GitHub link moved IR-1022 to done when spec PR #771 merged, which is why the "Correction: reopened" ticket comment exists. The ticket's acceptance is installed Kani Verified for the healthy clause and Falsified with in-domain playback for the mutant. The PR's own measurement shows both TimedOut, so that acceptance is unmet. Fix: unlink or mark the PR as "Refs IR-1022" so merging does not complete the ticket, or have the lead reopen it immediately and record the unmet acceptance. | PR #776 title/branch |

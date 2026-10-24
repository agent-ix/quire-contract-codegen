---
id: SR-17000
title: "IR-645 CG #561 code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#561; Cargo.lock; src/kani/classify.rs, generate/corpus/bounded_kani_corpus.rs, generate/lower/*.rs, run/execute.rs, terminal.rs; tests/it/bounded_kani_corpus.rs, terminal_map.rs; FR-030, TC-041"
review_set: subset
---

## Summary

Ticket: IR-645. Reviewed exact head `0a6d44b71da57f51b9216305655a648096e22cf6` against `8d81a87c89fc61d9ec36d1bc40131b575560644f`. The accessor conversions preserve field reads. The terminal mapper reads IR's retained count, converts it with `u32::try_from`, and refuses overflow before returning a terminal value. The test constructs `Proved(3)` through IR and would fail the old `Proved(0)` mapping defect. No aggregate gate was run in this reviewer lane; the author reports focused corpus 7/7, library 26/26, terminal 32/32, Clippy and fmt.

## Examined criteria

- FR-030-AC-1 (examined): every representable outcome and settlement pair maps once, while an over-range count returns only a typed refusal.
- FR-030-AC-4 (examined): three retained successful checks become certified Checks(3), without a separate caller count; zero is refused and overflow is typed.
- FR-030-AC-5 (examined): vacuous Inconclusive maps to the zero-check category, while other causes are refused by IR.
- FR-030-AC-8 (examined): the two admitted Unavailable causes retain distinct terminal causes.
- FR-030-AC-18 (examined): a mapped proof basis comes from the retained count.
- FR-015-AC-52 (context_only): generated corpus case values are not Kani run verdicts.
- TC-041 (examined): terminal mapping, refusal, and no-wildcard controls.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The updated corpus constructs `KaniOutcome::proved(1)` from an emitted assertion before Kani runs. Its public `BoundedCorpusCase.outcome` can be passed to the public `ir_outcome_terminal_value`, which now reads that fabricated retained SUCCESS count and returns certified `Checks(1)`. The inline comment and IR-1002 tracking identify the inherited pre-run Boolean claim but do not prevent this new numeric evidence from being accepted as an observed proof. Keep an expected-case value distinct from a run outcome, or prevent this pre-run value from reaching certified terminal mapping. | src/kani/generate/corpus/bounded_kani_corpus.rs:507-515, src/kani/generate/corpus/bounded_kani_corpus.rs:343-347, src/kani/terminal.rs:187-212 |

## Verdict

**FAIL** at this head because a public pre-run corpus result can be certified with a fabricated retained check count. The unchanged pre-run Boolean claim is tracked by IR-1002; this PR adds the positive count needed by the new IR API. The checked conversion and other accessor changes are sound on genuinely observed outcomes.

## Dispositions

Disposition pass 1 at exact head `b7dedc2776608b5f84941af3ff609d9a2b78433a` (parent `0a6d44b71da57f51b9216305655a648096e22cf6`).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The public `BoundedCorpusCase` now holds `BoundedCorpusExpectation`, whose private fields and methods expose only a predicted assertion result and predicted count. The generator no longer constructs `KaniOutcome::proved` or `::counterexample`; `src/lib.rs` exports the expectation type and `src/replay/function.rs` consumes only its expected Boolean. No conversion from expectation to `KaniOutcome` exists, so the public terminal mapper cannot accept the generation-time value. The TC-023 source guard failed on the prior public outcome and now passes; the changed code has no production path from expected count to certified Checks. |

## Narrow delta review

**PASS** for the seven-file fix delta. FR-015-AC-52/91/95 and TC-023 now call the value an expectation, keep the selected revision, distinguish one or zero predicted checks from a run transcript, and retain FR-017 as the verdict source. This closes the pre-run proof-count path while preserving the typed refusal return on the interim corpus API. The author reports focused corpus integration 8 passed with 8 Kani-lane tests ignored, corpus library 26/26, TC-041 32/32, scoped Clippy, spec, and fmt; no aggregate gate was run by this reviewer.

## Review ID correction

This artifact supersedes the same-method IR-645 CG #561 reviewer record originally labeled with a colliding SR id. The finding and disposition substance is unchanged; only the review ids and cross-references were reissued.

## New findings (disposition pass 2)

At exact head `701558b382b8b9090349caac0a2807a199885e6d`, the only delta from reviewed custody head `e5528ea7` is `src/kani/terminal.rs` (4 insertions, 1 deletion). The new typed error replaces the panic on a missing retained proof count, and all valid IR-outcome behavior remains as previously reviewed.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The public `ir_outcome_terminal_value` now returns `TerminalPairError::MissingProvedCount` if `success_checks()` is absent, but its `# Errors` documentation still enumerates only settlement errors and proof-count overflow. Name the new defensive error in the public function contract; it does not change FR-030's valid-input mapping. | src/kani/terminal.rs:185-203 |

## Narrow delta verdict

**CONDITIONAL** pending FND-002 documentation. The defensive branch itself is sound: it returns a typed error rather than panicking, does not certify a missing count, and leaves the checked `u32` conversion and existing terminal mapping unchanged. The author reports focused TC-042 1/1, terminal map 32/32, scoped Clippy and fmt; no aggregate gate was run by this reviewer.

## Dispositions, pass 3

Exact head `0e5f278386e81f58cc9e33c0ef61de25c1d4a3b2`, parent `701558b382b8b9090349caac0a2807a199885e6d`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | The sole delta is the public `ir_outcome_terminal_value` `# Errors` sentence, which now names a proved outcome with no retained count alongside missing/unexpected settlement and overflow. No implementation, test, spec, or lock bytes changed. |

**PASS** for the narrow documentation fix. FND-001 remains fixed from disposition pass 1; no open findings remain in this code-review record. This review makes no new aggregate-gate claim.

## New findings (disposition pass 4)

Exact head `fc896ff69dd767101e608d712f60f6055faca3bc`, parent reviewed custody head `10b19eba`. The only delta is a test-only pidfd-confirmation owner constructor in `src/kani/run/owners.rs` and `tool.rs`, used by the unignored FR-017-AC-42 cancellation/timeout control. Normal `LaunchOwners::real` and the ordinary 5-second cleanup boundary are unchanged; memory and stream owners remain real in the test helper.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | `TestOnlyPatientPidfdPoll::poll` discards the owner's finite confirmation timeout and calls `poll(fds, None)`. `NamespaceOwner::cleanup` can therefore block indefinitely if a teardown regression leaves the owned pidfd unreadable. The unignored cancellation/timeout test then hangs the library suite instead of observing a typed cleanup refusal. Use a finite, host-tolerant test confirmation bound longer than the production 5 seconds; a timeout must still return control to the test. | src/kani/run/owners.rs:49-55, src/kani/run/namespace.rs:364-375, tests/it/kani_scalar_native_private.rs:124-218 |

## Narrow delta verdict, pass 4

**CONDITIONAL** pending FND-003. The private helper's other substitutions are confined to the test build and preserve genuine process memory, stdout, stderr and owned pidfd observation. The author reports focused AC-42 1/1, under-load control, AC-55 2/2, scoped Clippy and fmt; no aggregate gate was run in this review lane.

## Dispositions, pass 5

Exact head `3355357e9be50ba3dfa305202704dc14046be1fe`, parent `fc896ff69dd767101e608d712f60f6055faca3bc`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | The test-only confirmation now polls the genuine owned pidfd once with a finite 30-second timeout. If it remains unreadable, `NamespaceOwner::cleanup` receives a zero-ready result and returns its existing typed unconfirmed-teardown error. Normal `SystemPoll` and production's five-second timeout remain unchanged. The helper stays behind `#[cfg(test)]`; no test assertions or production bytes changed in this fix. |

**PASS** for the narrow two-file fix. All prior findings remain fixed. The author's focused AC-42 1/1, AC-55 2/2, scoped Clippy and fmt claims remain subject to the final-head gate; this reviewer did not run an aggregate gate.

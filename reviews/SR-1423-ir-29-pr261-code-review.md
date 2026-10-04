---
id: "SR-1423"
title: "CG PR 261 code review (with rust-review lane): pin the replay verdict to the exact witness values"
type: SpecReview
analysis: code-review
review_set: base
scope: "agent-ix/quire-contract-codegen@bff5abe9020d620e19f5f823ce158062d73f9349; tests/it/skeleton_spine.rs (diff origin/main...HEAD)"
---

# SR-1423: CG PR 261 code review (with rust-review lane)

## Summary

Ticket: IR-29. Test-only PR: one file, `tests/it/skeleton_spine.rs`, +68/-0. It adds
`point_source(a, b)`, a QSL native twin whose clause is
`amount_current < a or amount_current > a or balance_pre < b or balance_pre > b`, a default-lane
test `tc_026_the_replay_verdict_is_decided_by_the_exact_witness_values`, and a block in the
ignored Kani-lane test
`tc_026_one_boolean_clause_goes_from_a_bound_package_through_kani_to_native_replay` that builds the
point twin at the decoded witness and a neighbour twin one step away on `amount_current`.

What was measured, all at the head above, each mutation in a throwaway worktree with its own
target dir (removed afterwards):

- Default lane: the new test passes, with the other 26 default `tc_026` tests in `skeleton_spine`.
- Kani lane unmodified (real Kani 0.68): passes, 279.8 s. Kani's witness is (991, 993).
- (a) The coder's mutation: the real transcript replaced by `playback(&harness, 1000, 1000)` before
  the point block. The Kani lane fails at the point assertion ("the replay did not evaluate the
  decoded witness (991, 993)"). Confirmed.
- (b1) Extraction mutation the coder did not run: `decode_values` returns `value + 1` for every
  integer, applied after Kani's `//` comment cross-check. The default-lane test fails (`(1, 5)`
  transcript is not Reproduced). The Kani lane **passes** (398 s): FND-001.
- (b2) Extraction mutation: `decode_falsification` swaps the two decoded values, names kept. The
  default-lane test fails. The Kani lane fails, but only at the pre-existing precondition
  assertion `get("amount_current") <= get("balance_pre")`, because Kani happened to pick
  991 < 993. The new point assertions would not have caught it: they read the same swapped values.
- (c) The point twin built at `(amount, balance + 1)`: the Kani lane fails at the point
  assertion. Confirmed.

The point twin is exactly false at one point: the disjunction is false only when
`amount_current == a` and `balance_pre == b`, over integers, and the default test checks the four
axis neighbours and the swapped point `(5, 1)`. The Kani-lane twin is built from the decoded values
via `get`, never from a hard-coded 991 or 993, so the test does not depend on the solver's choice.

Rust-review lane: test-only code. Trace tags `FR-016-AC-9, TC-026` on the new test are a correct
binding. No new `unsafe`, no production panic surface (the `.expect` calls are in tests). The
closure-based `replay` helper and the `for` over neighbour tuples follow the file's conventions.
Formatting is rustfmt-clean (`make fmt-check` in `make ci`).

## Verdict

Mergeable. The PR does what it says for the replay end: in both lanes a constant, a substituted
transcript or a twin at the wrong point turns the test red, and that was run, not asserted. It
does not pin the extraction end in the Kani lane: the Kani lane takes its oracle from the same
decoder `replay_counterexample` uses, so a decoder that returns wrong values consistently goes
unnoticed there (FND-001, measured). The default lane does catch that mutation, because its
transcript is built by the test itself, so the suite as a whole is not blind to it. FND-001 should
be fixed in this PR if IR-29 ask 3 ("fail if either end is re-stubbed") is to be claimed for the
real-Kani lane. Otherwise record it on the ticket as open. FND-002 is a nit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Kani-lane point check derives `(amount, balance)` from `decode_falsification`, the same decoder `replay_counterexample` calls internally, so oracle and subject share the extraction step. A decoder that returns `value + 1` after Kani's comment cross-check leaves the Kani lane green (measured: 398 s, passed), and by the same reasoning a decoder stub returning any fixed in-domain pair with amount <= balance and amount > 0 would also pass. The swap mutation was caught only by the pre-existing precondition assertion, and only because Kani chose 991 < 993. Pin the extraction end against an independent read of the real transcript, e.g. assert that Kani's own `// {amount}` and `// {balance}` comment lines appear in persisted order, or re-read the little-endian byte vectors in the test | tests/it/skeleton_spine.rs:1185-1215 |
| FND-002 | low | The neighbour assertion accepts any `ReplayVerdict::EvidenceFailure(_)`, while the default-lane test pins `EvidenceFailure(Verdict { settlement: Inconclusive, category: Success })`. Asserting the exact cause here too would keep a decode or domain refusal on the neighbour path from passing as a non-reproduction | tests/it/skeleton_spine.rs:1216-1223 |

## Dispositions

Round 1, reviewed at 22ba1af5b25c01adf73c17455657a99c58c95a7c. The fixes are in 1f33679 and
7fc5fb9. I re-measured with real Kani 0.68. Each mutation ran in a throwaway worktree with its own
target dir, sequentially. Kani's witness was (991, 993).

- Unmodified head: Kani lane passes (157 s). All 27 default `tc_026` tests in `skeleton_spine`
  pass.
- (b1) The decoder returns value + 1. The Kani lane fails at `skeleton_spine.rs:1247`, "the
  decoder disagrees with the values Kani printed". The default lane also fails.
- (b2) The decoder swaps the two values. The Kani lane fails at the same line, 1247, which is now
  ahead of the precondition assert. The default lane also fails.
- Fixed-pair stub: the decoder returns (1, 5) for every transcript. The Kani lane fails at line
  1247. The default lane also fails, at `(0, 5)`.
- Constant transcript substituted after the printed witness is read (the coder's site). The Kani
  lane fails at line 1262, the point assertion: "the replay did not evaluate the witness Kani
  printed (991, 993)".
- Constant transcript substituted right after `Falsified`, before anything reads it. The Kani lane
  **passes** (320 s). Every reader then sees the same self-consistent synthetic block. This is
  expected rather than a defect of the test: what ties the transcript to Kani is the Verified,
  then Falsified outcome sequence, not the witness check. It does make one TC-026 sentence too
  broad: SR-1424 FND-004.

`printed_values` is independent of the decoder. It splits the test's own lines with
`strip_prefix("// ")` and `strip_prefix("vec![")`, with no regex and no call into
`src/kani/output/playback.rs` or `src/replay/witness.rs`. The one assumption it shares with the
decoder is little-endian byte order. That cannot hide a shared bug, because Kani's own decimal
`//` comments are a third source, and the test asserts comments == bytes before it compares
either with the decoder. A transcript with any extra `// <int>` line, such as a second block,
fails loudly at `let [amount, balance] = printed[..] else`.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 7fc5fb9 |
| FND-002 | fixed | 1f33679 |

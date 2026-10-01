---
id: "SR-633"
title: "IR-92 slice 2 code review: Kani playback decode onto qsl-replay WitnessValue"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@92ec95afe85b8af23e3123105d9e7120a5b452ff; src/kani_witness_join.rs, src/spine_replay.rs, src/lib.rs, src/kani_transcript.rs, tests/it/kani_witness_join.rs, tests/it/skeleton_spine.rs, Cargo.lock"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
---

# SR-633: IR-92 slice 2 code review

## Summary

Ticket: IR-92 (slice 2). PR: agent-ix/quire-contract-codegen#204, head 92ec95a, base bb8523f.
This review covers code-review with the rust-review lane folded in. It is scoped to
`git diff origin/main...HEAD`.

## Method

I read every changed file in full. I compared the new decoder in `src/kani_witness_join.rs`
line by line with the IR parser it replaces (`quire-contract-ir` `src/kani/witness.rs` at the
locked 54f9a48) and with QSL's `qsl-replay/src/witness.rs` on QSL main (bad4944, identical in
that file to the locked a8e15b7). I ran `make ci` at the head with a private TRUSTED_HOME, and it
exited 0. I ran 13 source mutations against the decoder's unit tests. I generated real Kani
0.68.0 concrete-playback output for a three-value harness (i64::MIN, a bool and -2) and for a
cover-plus-assertion harness, and I decoded it through `decode_falsification` in a temporary
test. I ran the two ignored real-Kani TC-026 tests (`kani_witness_join`, `skeleton_spine`), then
re-ran them with a one-line temporary fix to confirm the cause. Every temporary edit was reverted,
and the worktree was clean afterwards.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `check_clause` returns only the rest of the `Check for` line, so `read_block` looks for the check text's closing quote on that one line. Kani's contract-harness check text spans lines (the contract closure is printed verbatim with its newlines), so every real contract-harness transcript fails with `kani_witness_check_text_missing`. As a result, `replay_counterexample` reports every real counterexample as `EvidenceFailure(Decode)`. Both ignored real-Kani TC-026 tests fail at this head (`tests/it/kani_witness_join.rs:405`, `tests/it/skeleton_spine.rs:620`). Making `after_kind` the rest of the block, as IR's `locate_check_kind` did, makes both pass. This is a regression that `make ci` cannot see. | src/kani_witness_join.rs:187-200, src/kani_witness_join.rs:266-270 |
| FND-002 | medium | Half of the re-homed parser's refusal paths have no test in CG. These mutants survive the unit tests: the width check relaxed to `>` (a short i64 vector then reaches `copy_from_slice` and panics), an unknown check kind treated as cover, an unparseable byte read as 0, a missing comment tolerated, and a missing check text tolerated. `kani_witness_harness_missing`, `check_missing`, `concrete_vals_missing`/`_malformed`, `comment_missing` and `byte_invalid` are asserted nowhere in CG. Negative values and i64::MIN are not decoded in the default lane either. These paths were covered by IR's `tests/it/kani_replay.rs`, which IR-347 deletes along with `witness.rs`. | src/kani_witness_join.rs:440-750 |
| FND-003 | low | `WitnessSchemaError` is still re-exported as public API, but no public function returns it any more. `argument_types` is private and `decode_falsification` maps the error into `DecodeFailure`. The PR body calls it "the private schema refusal". It should be `pub(crate)`, or folded into `DecodeFailure`. | src/lib.rs:132, src/kani_witness_join.rs:62-77 |
| FND-004 | low | A test doc still attributes the harness-identity gap to `Witness::parse`/`Witness::decode`, which CG no longer calls. The PR body's claim that the decoder is "a fresh compact one, not a copy of IR's" is inaccurate. It is a behavioural port of IR's `witness.rs`: the same 16 codes, the same block selection, and `boolean_comment` verbatim. That is acceptable as a move, because IR-347 deletes IR's copy, but until then the parser lives in both repos. The doc should say so rather than deny it. | src/kani_witness_join.rs:601-605 |

## Verdict

Not mergeable. FND-001 breaks decoding of every real Kani contract-harness counterexample. The
fix is a few lines: return the remainder of the block after the check kind, not the remainder of
its line. Add a unit test with a multi-line check text copied from the real transcript shape
(the IR-211 transcript prints it). FND-002 should be fixed in the same round, because this PR
makes CG the owner of the parser.

What is right:
- The premise holds. QSL's `Witness::parse`/`decode` read QSL's own
  `<<<kind|harness|check|name=value;...>>>` grammar keyed by `WireNodeId`. No QSL API reads
  Kani's byte-vector playback, so CG has to parse it.
- On real Kani 0.68.0 output the decoder is correct everywhere except FND-001. It decodes
  i64::MIN, a `1` bool and -2 in position order, and selects the assertion over a cover. It
  refuses two assertions, a two-byte bool, a seven-byte i64, a byte of -1 or 256, a bool byte of
  2, a missing first comment, a disagreeing comment and an unknown check kind, each with IR's
  code. Trailing data after the block is ignored, as it was in IR.
- Every one of the 16 old error codes is kept, with the same source_id and context values. Only
  `KaniOutcomeKind` is dropped, and `spine_replay` was already discarding it.
- There are no `unwrap`, `expect` or panic paths in the decoder. Slicing is on ASCII marker
  boundaries, and `copy_from_slice` is guarded by the width check.
- `spine_replay` now passes `DecodeFailure` through unchanged.
- IR is not touched. CG reaches QSL only through qsl-replay. Nothing is vendored, and no
  compatibility layer is added.
- The Cargo.lock change moves every QSL git crate to a8e15b7 together, which is the repo's
  branch=main lock convention. `make deny`, including the one-copy check, passed. There is no pin
  ceremony.

Mutation results: 8 of 13 killed (bool any-byte, comment check dropped, big-endian i64,
multiple assertions allowed, identity check dropped, arity check dropped, cover as other,
non-argument binding accepted). The 5 survivors are listed in FND-002.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The check text's `:` is not anchored to the kind's closing backtick. Because `after_kind` is now the rest of the block, `split_once(':')` finds the first colon anywhere below, typically `let concrete_vals:`. So the colon lookup almost never refuses anything, and the "colon lookup dropped" mutant survives the tests. The coder's claim that this mutant is EQUIVALENT is wrong. A check line with quoted text and no colon (``/// Check for `assertion` "no colon"``) is refused by the code (`kani_witness_check_text_missing`) but decoded by the mutant. The refusal happens only because the lookup ran on to `concrete_vals:` and found no quote after it. Kani always prints `: "`, so nothing real is affected. Requiring `: "` directly after the backtick would make the check mean what it says. | src/kani_witness_join.rs:276-280 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6feece1: `check_clause` now returns the rest of the block after the kind. The "HIGH revert" mutant turns `decode_falsification_reads_a_multi_line_check_text` red. Both ignored real-Kani TC-026 tests pass at 6feece1 (reviewer run). |
| FND-002 | fixed | 6feece1: all 16 codes are asserted, plus negative, i64::MIN and i64::MAX. All 5 former surviving mutants are now killed, and 14 of 15 mutants are killed in total. |
| FND-003 | fixed | 6feece1: `WitnessSchemaError` is private and is no longer re-exported from `lib.rs`. |
| FND-004 | fixed | 6feece1: the test doc no longer names `Witness::parse`/`decode`. The module doc and PR body now call the decoder a behavioural port of IR's `Witness`. |

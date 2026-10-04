---
id: "SR-1424"
title: "CG PR 261 gap analysis: IR-29 witness round-trip, FR-016 and TC-026 against the tests"
type: SpecReview
analysis: gap-analysis
review_set: base
scope: "agent-ix/quire-contract-codegen@bff5abe9020d620e19f5f823ce158062d73f9349; tests/it/skeleton_spine.rs, tests/it/kani_witness_join.rs, tests/it/bounded_kani_corpus.rs, src/replay/witness.rs, src/replay/function.rs, spec/replay/functional/FR-016-witness-native-replay.md, spec/replay/matrix/TC-026-witness-native-replay.md, spec/replay/matrix/tests.md"
---

# SR-1424: CG PR 261 gap analysis

## Summary

Ticket: IR-29. Plan completion: not assessed. The PR changes tests only and claims no spec change
is needed.

Stale-claim measurement, re-done independently. IR's FR-031-AC-3 no longer exists on IR
`origin/main`: the FR-031 matrix row lists AC-1 only, and no TC-054 or TC-221 row exists. The
discarding tests the ticket quotes (`tests/kani_replay.rs`, `src/bounded_kani_replay.rs`) are not
in either tree. In CG, the requirement lives in FR-016. Its matrix rows are FR-016-AC-1..5, 8..11
and 13..23 Covered by TC-026, AC-6, AC-7 and AC-12 Planned, and the TC-026 summary row is
Planned. I spot-checked AC-3, AC-4, AC-9 and AC-10 against their tagged tests. They are backed.

Can any remaining test pass with a discarded witness, a constant or a literal?

- `tc_026_a_falsifying_input_replays_through_qsl_to_the_same_violation` and
  `tc_026_an_in_domain_counterexample_the_twin_falsifies_is_reproduced` would each pass on their
  own with any constant that has amount > 0. They are paired with
  `tc_026_the_replayed_verdict_depends_on_the_witness_value` (amount 0), which separates only
  amount 0 from non-zero. Before this PR, a replay that ignored `balance_pre`, or any constant with
  amount > 0 on the counterexample path, would have passed. The new default-lane test closes that
  for `replay_counterexample`, and the two paths share `replay_falsification_through`.
- `kani_witness_join::tc_026_real_falsification_decodes_against_the_persisted_schema_and_mutation_refuses`
  (Kani lane) asserts names, positions, `amount_current > 0` and growth. Its values are checked
  only through the decoder under test, the same weakness as SR-1423 FND-001.
- `bounded_kani_corpus.rs` builds no counterexample packet and calls no replay, consistent with
  TC-023's statement.
- The `src/replay/witness.rs` unit tests decode synthetic transcripts against exact expected
  values. They pin the decoder, independently of the replay.

No matrix row claims more than a test backs. AC-9's wording ("the verdict depends on the witness
value") is now backed more strongly than it states.

Whether IR-29 is done, by its own asks:

- Ask 1 (honest status): done. Measured as above.
- Ask 2 (extract, reconstruct, native agreement, driven by real `cargo kani`): built, as CG's
  decode, then `qsl_replay::replay` of a QSL twin. The "native" side is a hand-mirrored QSL twin,
  not a function derived from the contract, and not the Rust subject run at the witness. TC-026
  says so honestly.
- Ask 3 (fail if either end is re-stubbed, shown by running it): the replay end holds in both
  lanes (measured: constant transcript and wrong-point twin both go red). The extraction end holds
  in the default lane, where the witness source is a synthetic `playback()` transcript built by the
  test and decoded by the real decoder (off-by-one and swap both go red). It does not hold
  independently in the Kani lane (SR-1423 FND-001). The Kani lane is also `#[ignore]` and outside
  `make ci`, so neither real-Kani mutation is gated in CI.

## Verdict

The PR's test claims hold and no spec text is wrong. IR-29 should not be closed on this PR alone.
It can close once SR-1423 FND-001 is fixed, or once the ticket records the Kani-lane extraction
gap and the hand-mirrored-twin boundary (FND-001 below) as the explicit residue. FND-002 and
FND-003 are documentation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | IR-29 ask 3 is only partly backed. Re-stubbing the extraction end is caught by the default lane (synthetic transcript), but not by the real-Kani lane, whose oracle is the decoder itself. The "native" replay evaluates a hand-mirrored QSL twin rather than a function derived from the contract or the Rust subject, and the Kani lane is ignored, outside `make ci`. The ticket cannot be closed as "either end, end to end" without recording this residue or fixing SR-1423 FND-001 | spec/replay/matrix/TC-026-witness-native-replay.md:75-82 |
| FND-002 | low | TC-026 Status says the Kani lane "replays a real prover counterexample the same way" and does not record what this PR adds: the verdict is pinned to the exact decoded witness values by a twin that is false at a single point, with a neighbouring-point control. One sentence would let the matrix reader see that the witness is not discardable | spec/replay/matrix/TC-026-witness-native-replay.md:75-82 |
| FND-003 | low | Already on main, outside the diff: TC-026 Status, FR-016, FR-024, AD-001 and AD-003 cite `src/kani_witness_join.rs` and `src/spine_replay.rs`, which no longer exist. The code lives in `src/replay/witness.rs` and `src/replay/function.rs`. Belongs in a follow-up, not this PR | spec/replay/matrix/TC-026-witness-native-replay.md:85, spec/replay/functional/FR-016-witness-native-replay.md:51 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The new TC-026 Status sentence says that replacing the real transcript with a constant turned the Kani lane red. That holds only where the coder substituted it, after the printed witness is read. A constant substituted right after `Falsified`, before any reader, leaves the Kani lane green (measured, 320 s), because the printed reader, the decoder and the replay all see the same self-consistent block. The sentence should name the site, or say that the witness check pins the replay to the transcript it is given, while the link from the Kani run to that transcript rests on the Verified-then-Falsified outcome assertions | spec/replay/matrix/TC-026-witness-native-replay.md:82-91 |

## Dispositions

Round 1, reviewed at 22ba1af5b25c01adf73c17455657a99c58c95a7c. The spec text is in 1f33679 and
22ba1af. `grep -rn "kani_witness_join\|spine_replay\|frame_replay" spec`, excluding AD-004, now
matches only `TC-026:102`, which names `tests/it/kani_witness_join.rs`, a real test file. I agree
it stays, and I agree AD-004 keeps the old names, because it is the migration plan whose subject
they are. The new citations check out: `src/replay/frame.rs:47` holds the `[u8; 32]`,
`LockedSource::digest` is the only `ByteDigest::of` in `src/replay`, and `witness.rs` imports
`select_assertion_block` and `read_block` from `kani::output::playback`.

Outside FND-003 and not introduced by this PR: `kani_transcript` (now `src/kani/output/`) is still
cited by the spec.md Kani row, AD-001:48, 110 and 151, and AD-003:58 and 249. That belongs in a
separate follow-up.

The unbacked list in the TC-026 Status and the PR body is complete and honest: a hand-mirrored QSL
twin and a hand-built package, the `#[ignore]` Kani lane outside `make ci`, a default suite pinned
only against a synthetic transcript, and FR-016-AC-6, AC-7 and AC-12 planned. The PR says "Part
of IR-29" and that IR-29 should not close on it alone. That is correct.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1f33679 |
| FND-002 | fixed | 1f33679 |
| FND-003 | fixed | 22ba1af |

Round 2, reviewed at 6168bfbe4f53b67549e6cc84c5db3847962ffdc7. The only change over 22ba1af is
the TC-026 Status paragraph. It now names the substitution site ("replacing the transcript handed
to the replay with a constant after the printed witness is read"). It says that the witness check
pins the replay to the transcript it is given. It says that a constant substituted at the source,
right after `Falsified`, stays green. It says that the link from the Kani run to the transcript
rests on the `Verified` then `Falsified` outcome assertions. All of this matches what was measured
in round 1. The PR body's new "Scope of the witness check" paragraph and its mutation bullet say
the same.

I grepped `spec/` for "constant transcript", "with a constant", "turned the Kani lane red",
"cannot be discarded" and "any witness other". The only hits are the corrected TC-026 lines, so
the overclaim appears nowhere else. `quire coverage --strict` at this head: 65 unbacked,
0 contradicted.

`make spec` cannot be confirmed at this head right now. The installed quire-cli changed to
0.36.1 at 18:19 UTC during this pass, and since then every document of the process module's types
fails as "no archetype registered": 211 at this head and 220 on `origin/main`. That is an
environment regression, not this PR: the edit is prose inside an existing Status section.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 6168bfb |

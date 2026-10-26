---
id: SR-593001
title: "IR-348 PR #593 code-review with rust-review lane (typed witness and scalar replay)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #593 head 3ee11b66a9ed8ec6abd3d59b1d154a06e993844d on merge-base 68fafff7 (origin/main 8db3554e); src/lib.rs, src/replay/scalar.rs, src/replay/witness.rs, tests/it/kani_obligations_state_frame.rs, tests/it/kani_scalar_native_private.rs, tests/it/scalar_playback.rs; IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

## Summary

Ticket: IR-348. This is the independent Opus review of draft PR #593 at head 3ee11b66, scoped
to `git diff origin/main...HEAD`: one commit and seven files. The Rust lane (`rust-review`) is
folded into this artifact. The PR is one L-6 slice of AD-004 step 5. `decode_v2_clause_falsification`
now takes `&Playback<'_>` instead of transcript text. The public combined parser
`parse_scalar_playback` is deleted, along with its re-export, so `decode_scalar_falsification` is
the only public scalar entry. Callers now parse once with `parse_assertion_playback`.

What was checked and found correct:

- `Playback` fields are all `pub(crate)` (`src/kani/output/playback.rs:189-197`). An outside
  caller cannot forge `source_len`, `harness` or `entries`, so a typed entry is no weaker than the
  transcript entry it replaces.
- `decode_v2_clause_falsification` still checks provenance first. It passes the caller's
  `limits` to `decode_playback`, which re-checks `source_len` against `limits.bytes` and the entry
  count against `limits.values` (`src/replay/witness.rs:254-283`). A caller that parsed with a
  looser limit is therefore still refused. The decoder's inclusive-domain check is unchanged.
- `decode_scalar_falsification` still validates the schema, then its own 8 MiB `source_len`
  ceiling, then harness identity, arity, width and comment agreement, then domain
  (`src/replay/scalar.rs:35-87`).
- No other caller of the removed or retyped functions exists in `src/`, the specs, the docs or
  the tests (grep at head). The remaining `parse_assertion_playback` call sites under
  `src/replay/` are in `function.rs`, `state_clause.rs` and `frame.rs`, which matches the AD-004
  residual list exactly.
- No new hash, pin, compatibility shim or cross-repo copy was added. Removing the public
  `parse_scalar_playback` is a prerelease API removal with no shim, which is correct.
- No `unsafe`, panic path, `todo!`, or unbounded work was added. The `.expect` calls are in tests
  only.

Gates: no Cargo gate was run, per the brief (a shared final gate is running). The coder's
focused logs at `qualification-logs/p-ir-79-code-lead/cg593-focused-{clippy,witness,scalar,v2}.log`
show clippy `--lib --test it` passing, `replay::witness` 15/15, `scalar_playback::` 10/10 and
`native_v2_decoder_enforces_original_inclusive_bounds` 1/1. The two native Kani mutant tests
edited in `kani_scalar_native_private.rs` and `decode_native_v2_helper_mutant_playback` have no
focused log, so their edits were reviewed statically only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The scalar decoder's own 8 MiB ceiling is no longer exercised by any test, yet it is now the only scalar-owned byte guard. `decode_scalar_falsification` refuses `playback.source_len > PLAYBACK_BYTES`, but callers now choose the parse limit. The test helper `decode_from_transcript` parses at `PLAYBACK_BYTES`, so the `oversized` case in `typed_schema_byte_ceiling_decode_and_domain_refusals` is refused by the test's own `parse_assertion_playback` call, not by product code. The native tests also parse at 8 MiB. Deleting `scalar.rs:43-48` fails no test, and a caller that parses at 16 MiB then passes an 8 MiB + 1 transcript straight through. Add a test that parses above the ceiling with a looser reader limit and asserts `cg_witness_decode_bytes_limit` comes from the decoder. | src/replay/scalar.rs:43-48; tests/it/scalar_playback.rs:24-37; tests/it/scalar_playback.rs:431-434 |
| FND-002 | low | The doc comment says the transcript "is limited to 8 MiB by the output reader". The output reader enforces whatever `max_bytes` its caller passes. The 8 MiB limit is enforced by this decoder, on `source_len`. With `parse_scalar_playback` gone, the sentence now names the wrong guard. | src/replay/scalar.rs:19-20; src/kani/output/playback.rs:364-376 |
| FND-003 | low | The 8 MiB scalar ceiling is now spelled in four places: the private `PLAYBACK_BYTES` in `scalar.rs`, a separate `PLAYBACK_BYTES` in `tests/it/scalar_playback.rs`, and two `8 * 1024 * 1024` literals in `kani_scalar_native_private.rs`. That file compiles in-crate through `#[path]` from `src/routed/mod.rs:19`, so it could use the crate's constant (made `pub(crate)`). Changing the ceiling would leave the native tests parsing at the old value without any failure. | src/replay/scalar.rs:14; tests/it/kani_scalar_native_private.rs:96-102; tests/it/kani_scalar_native_private.rs:140-146; tests/it/scalar_playback.rs:22 |

## Verdict

The slice is correct and mergeable after FND-001. The typed boundary is sound because
`Playback` cannot be forged outside the crate. The V2 clause entry keeps its provenance, limit
and domain refusals, and the removed text-taking entries leave no callers or stale references.
FND-002 and FND-003 are low and can be fixed in the same round.

Rust gates were not run by this reviewer, per the brief. The coder's focused logs pass. The two
native Kani mutant tests the PR edits have no recorded run at this head, and the shared final
gate must cover them.

## Dispositions

Round 1. Checked at head b7fed787eb2e3125163f791d4b80a74cfb4fb9ea against fix diff 3ee11b66..b7fed787 (one commit, `IR-348: exercise scalar decoder limit and correct trace claims`). Cargo was not run by this reviewer. The coder's logs `cg593-fix-clippy.log` (clippy finished clean) and `cg593-fix-scalar.log` (`scalar_playback::` 11/11, including the new test) pass. Neither log records the head it ran at.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | New test `scalar_decoder_rejects_playback_above_its_limit_after_a_loose_parse` (`tests/it/scalar_playback.rs:451-472`) parses an 8 MiB + 1 transcript with the reader limit at `SCALAR_PLAYBACK_BYTES + 1`, then asserts that `decode_scalar_falsification` refuses with `cg_witness_decode_bytes_limit` and context `{module};bytes=8388608`. Deleting the decoder's check at `scalar.rs:45-50` would now fail it. |
| FND-002 | fixed | `scalar.rs:21-22` now reads "limited to 8 MiB by this decoder, even if the output reader accepted a larger input." |
| FND-003 | fixed | The ceiling is now the single documented `pub const SCALAR_PLAYBACK_BYTES` (`scalar.rs:15`, re-exported at `lib.rs:272`). The two native literals and the separate integration-test constant now use it. Making it `pub` rather than `pub(crate)` is required, because `tests/it/scalar_playback.rs` is an external integration test, and the constant documents the decoder's contract. |

## Coverage

```yaml
scope:
- id: src/replay/witness.rs
  path: src/replay/witness.rs
  role: examined
  excerpt: "pub fn decode_v2_clause_falsification(harness: &KaniV2ClauseObligationHarness, playback: &Playback<'_>, limits: WitnessDecodeLimits)"
- id: src/replay/scalar.rs
  path: src/replay/scalar.rs
  role: examined
  excerpt: "pub fn decode_scalar_falsification(identity: &ScalarObligationIdentity, playback: &Playback<'_>) -> Result<Vec<i64>, DecodeFailure>"
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: "pub use replay::scalar::decode_scalar_falsification;"
- id: tests/it/scalar_playback.rs
  path: tests/it/scalar_playback.rs
  role: examined
  excerpt: "fn typed_schema_byte_ceiling_decode_and_domain_refusals()"
- id: tests/it/kani_obligations_state_frame.rs
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: "quire_contract_codegen::decode_v2_clause_falsification(harness, &playback, limits)"
- id: tests/it/kani_scalar_native_private.rs
  path: tests/it/kani_scalar_native_private.rs
  role: examined
  excerpt: "let playback = crate::kani::output::playback::parse_assertion_playback(&counterexample, ..., 8 * 1024 * 1024, usize::MAX)"
- id: src/kani/output/playback.rs
  path: src/kani/output/playback.rs
  role: context_only
  excerpt: "pub struct Playback<'a> { pub(crate) source_len: usize, pub(crate) harness: &'a str, ... }"
```

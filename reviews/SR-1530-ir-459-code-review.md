---
id: "SR-1530"
title: "CG PR 284 code review (with rust-review lane and test-oracle strength): the minted frame obligation identity and the frame witness decoded from the real playback"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@528f46bad4abf8f2dbcd46672ce54a75f9cd00b2; src/core/identity.rs, src/kani/generate/frame.rs, src/kani/identity.rs, src/kani/test_support.rs, src/lib.rs, src/replay/frame.rs, src/replay/obligation.rs, src/replay/witness.rs, tests/it/kani_batching.rs, tests/it/kani_obligations_state_frame.rs, tests/it/terminal_map.rs, tests/state_frame_support/native_twin.rs (diff 735e704...528f46b); context: src/replay/state_clause.rs, src/replay/function.rs, tests/it/layout.rs, Cargo.toml; QSL agent-ix/quire-spec-language at c8f0c28 and bcca433"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
---

# SR-1530: CG PR 284 code review

## Summary

Ticket: IR-459. PR: agent-ix/quire-contract-codegen#284 at 528f46b, one commit on merge base
735e704. The PR text and the author's report were treated as untrusted claims. Everything below
was measured on the head.

- Identity. `function_contract_identity` in `src/replay/obligation.rs` is still the only minting
  function. `frame_identity(site, kind)` calls it with `site.frame`, `&site.frame_occurrence`,
  `kind` and `&[]`, so E-1 is not widened: a frame has empty arguments. `frame_kind` returns
  `None` for a postcondition. The existing function-path golden vectors are unedited: the
  `obligation.rs` diff only adds module docs, two helpers and new tests at the end of the test
  module. `sha2` appears only under `[dev-dependencies]`.
- Check order in `FrameReplay::new`. Before `call_site`, it checks `frame_kind` (NotAFrame), then
  `field_set_matches`, then the operation scope, then `decoded_state` (decode and domain). After
  `call_site`, it checks the anchor, then the frame, then `tie_pre_state` (a read-only check of the
  provided documents, which builds nothing), and only then mints. This matches FR-024-AC-22 and
  AC-25 to AC-28.
- Witness. `decode_playback` is now the one decoder and `decode_falsification` delegates to it.
  The transcript is `render_witness(harness_path, check_text, bindings)`. The fixed
  `<<<assertion|op|frame|>>>` text is gone.
- Layout and NFR-005. `replay` imports only `kani`, `core` and `replay`. `tests/it/layout.rs`
  passes. New re-exports are only in `lib.rs`. The non-test source adds no `unwrap`, `expect`,
  `panic!`, indexing, `unsafe` or `as`. No stand-in digest literal is in non-test `src`. The only
  `[1; 32]` left is the state-clause twin input, which Q-1 leaves out of scope.
- Public API. `Deserialize` is added to `StateFrameIdentity` and its member types with
  `deny_unknown_fields`. The symbols deserialize through `try_from = "String"`, so validation is
  kept. `StateFrameIdentity::from_record`, `StateFrameRecordError`, `ScopeMember` and
  `PreStateFault` are exported. `FrameReplayInputs` loses `obligation_identity`, which breaks the
  API on purpose (prerelease). No shim or compatibility layer is added. Its only consumers are the
  twin and the tests.
- Generated artifacts. The harness `.rs` text is unchanged. `render` adds `state_fields` to the
  identity, and the identity is serialized only into the persisted `kani-obligations/<module>.json`
  record. The record therefore changes for every state-frame harness, both frame and
  postcondition, and a record written before this PR is refused by `from_record`. Draw order:
  the harness builds `S { f1: kani::any(), f2: kani::any(), .. }` in `request.state_fields` order,
  and Rust evaluates struct-expression fields in source order, so `state_fields` is the draw
  order.
- Gates. I ran `make ci` on 528f46b with my own target dir and read the whole log
  (`ci-cg284-sr1530.log`). Exit 0: fmt-check, spec, clippy `-D warnings`, msrv, deny,
  audit-unsafe, rustdoc and test. lib 172 passed, it 385 passed with 25 ignored, doctest 1, the
  same under msrv and stable. These match the author's counts. I did not run `make kani` (host
  lock).
- Merge with current main (33d27e2, QSL lock bcca433). `git merge-tree` reports no conflicts. On
  the merge preview, `cargo test --locked` gives 172, 385 (25 ignored) and 1, clippy `-D warnings`
  is clean, and `quire coverage --strict` reports 44. `OperationSite` and `WitnessValue` are
  unchanged between c8f0c28 and bcca433.
- Test-oracle strength. I applied seven mutants by hand, reverted each, and left the worktree
  clean. Every one was killed by an assertion, not by a compile error:
  - M1, `frame_identity` uses `site.anchor` as `function`: killed by both new `obligation` unit
    tests.
  - M6, kind ignored: killed by `tc_035_a_frame_identity_is_the_one_function_over_the_frame_and_its_occurrence`.
  - M7, the harness clause node used as `function`: killed by four `kani_obligations_state_frame`
    tests (hand-written identity, names-none-of-the-harness-members, changed grant,
    follows-call-site).
  - M2, frame scope check removed: killed by the scope-by-member test.
  - M5, operation check disabled: killed by the same test.
  - M3, pre-state tie removed: killed by three tests (own pre state, unreadable pre state, draw
    order).
  - M4, exclusive maximum: killed by the endpoints test.

## Verdict

APPROVE (code), with four low findings. The minting, the check order, the decode and render reuse
and the refusals are correct, and the tests kill every mutant above. The low findings are on
edges: unverified document bytes read before QSL's digest check, a duplicated stringly re-spelling
of the snapshot shape, a duplicate-field hole in `field_set_matches`, and two members AC-21 names
that the insensitivity test never edits. The medium gap about the rebased node ids is in SR-1531.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `tie_pre_state` reads the invocation and pre-snapshot bytes by their claimed digest before QSL checks them against it. A provided document whose bytes do not match its digest can then return `PreState` (`Failed`) where QSL would refuse with `byte-digest-mismatch` (a catalog code, read as `Inconclusive(ReplayRefused)`). | src/replay/frame.rs:391-480 |
| FND-002 | low | `pre_object` and `tie_pre_state` spell QSL's snapshot and invocation shape again as untyped `Value` lookups (`pre.digest`, `self.population/key`, `populations[].objects[].fields.<f>.integer`). `state_clause.rs` already types that same shape (`SnapshotDocument`, `InvocationDocument`, `IntegerValue`, `ObjectRef`, `SnapshotLink`). Two spellings can drift. | src/replay/frame.rs:401-480; src/replay/state_clause.rs:572-631 |
| FND-003 | low | `field_set_matches` compares sorted multisets, so `state_fields [a, a]` with `granted [a]` and `checked [a]` passes. Its own doc says a field named twice refuses. Neither `from_record` nor `decode_playback` rejects a repeated field. | src/replay/frame.rs:316-333 |
| FND-004 | low | The AC-21 insensitivity test edits no harness symbol and no solver, although the AC names both and the test's own doc names the harness symbol. Its two-postcondition-clauses case sets `harness.clause` to `scope.object` and never mints from a unit with one clause on `deposit` against one with two. | tests/it/kani_obligations_state_frame.rs:2241-2351 |

## Dispositions

Round 1, reviewed at 8f7db6cbc47dc8738d76d8035849ffbf126043ef (fix commits 1440e9f and 8f7db6c on the merge of main c99f4c6, whose tree equals the clean merge of 528f46b and 33d27e2). `make ci` was re-run on 8f7db6c with my own target dir and the whole log read (`ci-cg284-disp1-8f7db6c.log`): exit 0, lib 173, it 387 with 25 ignored, doctest 1, the same under msrv and stable. `quire coverage --strict` reports 44. `git merge-tree` with main 4502a21 has no conflicts. I applied three new mutants by hand and all three were killed: the request decode dropped, a duplicate binding allowed, and a repeated record field allowed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1440e9f: `FrameReplay::new` now runs `qsl_replay::ReplayRequest::decode` on the request before `tie_pre_state`. QSL bcca433 `request.rs` recomputes every byte-provision entry: `PackageDocument::parse` then the JCS digest for `sha256-jcs`, refusing `ContentMismatch` or `ByteDigestMismatch`, both coded `stale_dependency`. Measured: both twin documents, given mismatched bytes, refuse `Request(ContentMismatch)` with code `StaleDependency`. `replay_frame` runs the same decode first, so a valid request behaves as before; only the refusal now surfaces from `new`. |
| FND-002 | fixed | 1440e9f: the reader deserializes `ObjectRef`, `SnapshotLink`, `SnapshotPopulation`, `SnapshotObject` and `IntegerValue` from `state_clause.rs`, which gained `Deserialize`, so one definition covers writing and reading. 8f7db6c moves `ProvidedDocument` into `function.rs` to break the frame and state-clause import cycle. The layout tests pass, including `l_2_the_module_graph_is_acyclic`. |
| FND-003 | fixed | 1440e9f: `decode_playback` refuses `cg_witness_schema_duplicate_binding`, and `from_record` refuses `StateFrameRecordError::RepeatedField`. Both are tested, and mutants N2 and N3 are killed. |
| FND-004 | fixed | 1440e9f: the AC-21 test now edits the harness symbol and compares a one-clause unit with the two-clause unit. The solver is not edited because `KaniSolver` has one variant (`src/kani/abi.rs:53`), and the test's doc says so. |

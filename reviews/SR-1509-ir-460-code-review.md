---
id: "SR-1509"
title: "CG PR 278 code review (with rust-review lane and test-oracle strength): StateClauseReplay"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@986f23cfa0bd371c73318ff285fbed44049af46d; src/replay/state_clause.rs, src/replay/function.rs, src/replay/mod.rs, src/core/canonical.rs, src/kani/terminal.rs, src/lib.rs, tests/it/kani_obligations_state_clause_replay.rs, tests/it/kani_obligations_state_frame.rs, tests/it/terminal_map.rs, tests/it/layout.rs, tests/it/main.rs, tests/state_frame_support/native_twin.rs, tests/state_frame_support/subject.rs (git diff origin/main...HEAD, merge base e526390 = origin/main)"
---

# SR-1509: CG PR 278 code review

## Summary

Ticket: IR-460. PR: agent-ix/quire-contract-codegen#278 at 986f23c, one commit on `origin/main`
e526390 (the merge base is `main`'s tip). The `rust-review` lane and the test-oracle (mutant)
check are folded into this file. The PR body and the author's report were treated as claims and
re-measured.

What I checked myself:

- `make ci` on 986f23c, in my own worktree and target dir: see Verdict for the result. `make kani`
  was not re-run (it holds the host lock for a long time and nothing found here needs it); the
  author's 24/0 claim for the real-Kani AC-18 test is therefore unverified by this review.
- Document members against QSL c8f0c28 (`qsl-semantics/src/model/observation/document.rs`, the
  rev in `Cargo.lock`). Snapshot: `format`, `identity`, `observation`, `model`, `populations`;
  population `population`, `complete`, `objects`; object `key`, `type`, `fields`; values
  `{"integer": "<decimal>"}` (QSL `read_raw_value`). Invocation: `format`, `identity`, `model`,
  `context`, `operation`, `self` (`population`, `key`), `pre`/`post` (`identity` with the four
  labels, `digest` `sha256-jcs:<64 hex>`), `parameters` (object), `result` (`null` accepted as
  `ResultValue::Null`), `created`, `deleted`. Model header `identity`, `version`, `digest` with the
  `sha256-jcs:` prefix. Every member CG emits matches the reader; no extra member.
- No second encoder: documents are `Serialize + FixedShape` records encoded by
  `core::canonical::{content_bytes, content_digest}` only; `content_bytes` is the one new function
  and wraps `quire_canonical::to_vec` with the same `PREIMAGE_LIMITS`. No `sha2`, no
  `ByteDigest::of`, no sort or escape in production code. `Cargo.toml` is not in the diff.
- Layout: `replay` may import `kani` and `core` (`tests/it/layout.rs` `may_import`); the module
  imports `core::canonical`, `kani::terminal` and `replay::{frame, function}`. Satisfied.
- NFR-005: the production part of `state_clause.rs` has no `unwrap`, `expect`, panic macro, index
  or slice; the `u32::try_from(position)` conversion is checked.
- Visibility: new `pub` items are exactly those `lib.rs` re-exports (`StateClauseReplay`,
  `StateClauseReplayInputs`, `StateClauseReplayError`, `DocumentError`, `DocumentLabel`,
  `StateObjectAddress`, `OperationDeclaration`); `render_witness`, `encode_document`,
  `EncodedDocument` are `pub(crate)`. The error variants match interface-001's
  `StateClauseReplay::new` entry. Error and struct conventions (no `source()`, no `Debug` on the
  replay struct, public `wire`/`packet`) match `FrameReplay`.
- QSL's `replay_state_clause` (c8f0c28, `qsl-replay/src/execute/state_clause.rs`) reads neither
  the witness transcript nor `declared_domains`; it recompiles, checks package id, clause
  identities and observation form, then admits the documents by FR-106, where a snapshot value
  outside the model's range is refused `invalid_runtime_input` / `invalid-value`.
- Generated harness (`src/kani/generate/frame.rs` `symbolic_state`): it assumes only the pre
  state's ranges; the post state is unconstrained.
- Existing frame tests: the diff to `kani_obligations_state_frame.rs` changes visibility, adds a
  `result` shape flag (default false), adds an unreferenced `RESULT` parameter node to every
  fixture package, and keeps the `self`/`other` binder order. No assertion was removed or
  loosened. The twin's QSL domain package lost its operations' `returns`, and the twin's frame
  invocation's `result` changed from `{"boolean": true}` to `null` (see SR-1511 FND-002).
- Merge risk with IR-461 PR #277 (head dbff533): `git merge-tree` of the two heads conflicts in
  one hunk pair of `tests/it/kani_obligations_state_frame.rs` (the `Shape` struct fields and
  `Shape::HEALTHY`: #277 adds `condition_field`/`member`, #278 adds `result`). The resolution is
  to keep both. `src/lib.rs` and `spec/kani/matrix/tests.md` auto-merge. Fixture variants do not
  collide (#277 uses up to 20, #278 uses 30 and 31). #277 touches `src/kani/generate/frame.rs`,
  not `src/replay/frame.rs`.

Probes run on my own worktree, each reverted (results in Verdict).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A post state outside the model's declared range is not detected by `StateClauseReplay::new` and surfaces as a QSL data refusal. `bind(&fields, &post_state, false)` skips the range check. The generated harness assumes only the pre state's range. So the seeded defect the merged AC-18 named (a debit, `deposit_debiting`, `wrapping_sub`) gives a real counterexample at balance 0, a native post state of -1, and QSL admission refuses the post snapshot (`InvalidRuntimeInput` / `invalid-value`). The error is `Refused(Admission(..))`, which FR-029-AC-16 reads as `Inconclusive(ReplayRefused)`. A real violation found by the prover is reported as inconclusive and blamed on a QSL data refusal, while the defect is the subject leaving the model (or CG's harness not bounding the post state). The PR changes the fixture and the AC text instead of the behavior (SR-1510 FND-001). Measured by probe: playback (0,0), post (-1,0) builds, replays and ends `Refused`. | src/replay/state_clause.rs:332-333, src/replay/state_clause.rs:689-713, tests/state_frame_support/subject.rs:37-50 |
| FND-002 | medium | The FR-024-AC-17 test cannot tell which side is range-checked. It writes the out-of-range value into both `playback` and `post_state` (`for values in [&mut candidate.playback, &mut candidate.post_state]`). A mutant that range-checks the post state instead of the playback (swap the `check_range` flags), or both sides, still returns `OutOfDomain` naming the field and passes. Probe confirmed. No test pins the claimed "playback only" rule or the out-of-range-post path of FND-001 | tests/it/kani_obligations_state_clause_replay.rs:380-409 |
| FND-003 | medium | The FR-024-AC-13 test runs over one value per caller input, so a mutant that hard-codes a document member survives. The twin supplies one object key (`account`), one population, one object type, and three labels that differ only in `identity` (authority `test`, namespace `ns`, revision `1` on all three). A `snapshot`/`InvocationDocument` that writes the literal `"account"`, the twin's population, or swaps `authority` with `revision_namespace` in `IdentityMember::from` passes every assertion, and QSL accepts it because the twin's inputs agree. Vary the address and the four label fields across calls, or assert each label field against a second distinct input | tests/it/kani_obligations_state_clause_replay.rs:247-305, tests/state_frame_support/native_twin.rs:440-477 |
| FND-004 | medium | A second package reader in `replay/state_clause.rs` diverges from the generator's reader of the same facts in `kani/generate/frame.rs`. The duplication is not forced: `replay` may import `kani` (layout `may_import`), so "replay may not import oracle" is not the constraint. Where they diverge: (a) `read_fields` requires every member of the framed object to reference an `integer_range`, else `Document(NoRange)` and `Failed`. `state_domains` bounds only the fields that declare one and generates the harness anyway, so a model with a plain-integer field the clause does not read gets a harness whose counterexamples can never replay. (b) `binding()` takes the first matching `min`/`max` and ignores extra or duplicate members, where `oracle::scalar::bound_members` refuses them. (c) `literal()` is a byte copy of `oracle::scalar::literal`. One fact (a field's range) now has two readers that can disagree on the same package | src/replay/state_clause.rs:775-790, src/replay/state_clause.rs:845-877, src/kani/generate/frame.rs:764-805 |
| FND-005 | low | `declared_domains` (one `DomainKey` per field: `self`'s node, path = the member's position in the framed object, the field's range) and the witness transcript's bindings are asserted by no test. QSL `replay_state_clause` reads neither, so a wrong path, a swapped min/max, or an empty transcript passes every test and every replay | src/replay/state_clause.rs:717-747, src/replay/state_clause.rs:365-370 |
| FND-006 | low | `bind` reads each declared field with `find` and ignores the rest. A playback or post state that binds a name the object does not declare is dropped silently, and a duplicated name takes its first value. Under FR-024's "check every decoded value against the declared domain of the parameter it binds", an undeclared binding has no domain and should be refused, or the rule should say it is ignored | src/replay/state_clause.rs:691-713 |
| FND-007 | low | `model_header` turns a supplied package that does not parse (`serde_json::from_slice(..).ok()?`) and two matching packages into the same `DocumentError::Model` ("no supplied domain package declares the model"). The message is misleading in both cases. It also copies the provided document's digest domain into the header, but QSL's `read_model` requires `sha256-jcs:`, so a package provided under any other domain ends as a QSL `wrong-value-kind` refusal (`Inconclusive`) rather than a CG error | src/replay/state_clause.rs:648-675 |
| FND-008 | low | Two smells, one fact in two places each. (a) `hex()` with `format!("{}:{}", DigestDomain::Sha256Jcs.as_str(), hex(..))` in `Snapshot::link` repeats what `DigestRecord::{domain, hex}` already gives `model_header` 70 lines below. (b) `encode_document` encodes each document twice (`content_bytes`, then `content_digest` re-encodes); `core::canonical` could return both from one encoding | src/replay/state_clause.rs:582-592, src/replay/state_clause.rs:465-472 |

## Verdict

FAIL: one high finding (FND-001) and three medium findings. Ran on 986f23c: `make ci` with my own
`CARGO_TARGET_DIR` exits 0. fmt-check, spec, lint, msrv and deny (advisories, bans, licenses,
sources ok), audit-unsafe and rustdoc pass. Tests: lib 164 passed, it 355 passed with 24 ignored,
doctest 1, in both the msrv and the test passes. That matches the author's numbers. `make kani` was
not re-run.

Probes on my own worktree, each reverted (`git status --porcelain` clean afterwards):

- Out-of-range post state: playback (0, 0), post (-1, 0). `new` succeeds, and `replay` returns
  `Refused(Admission(Refused(AdmissionRecord { code: InvalidRuntimeInput, cause: "invalid-value",
  fields: {"field": "balance", "object": "account"} })))`. `run_terminal_value(Falsified, 0,
  Some(settlement))` gives `Ok(Inconclusive(ReplayRefused(InvalidRuntimeInput)))`. This confirms
  FND-001.
- Swapped range check (playback unchecked, post checked): every state-clause test (8 run, the
  real-Kani one ignored) and `tc_040_the_state_clause_replay_reads_as_fr029_ac16` still pass. This
  confirms FND-002.
- Object key hard-coded to `"account"` in the snapshot and invocation: all 8 state-clause tests
  still pass. This confirms FND-003.

Mutants the tests do catch, by reading the tests: a faked verdict (AC-11 compares with the direct
QSL call over two runs that differ); a dropped range check (AC-17); a swapped settlement
(`tc_040`, `Refuted` against `ReplayParity`); a skipped shape check (AC-19 clears the playback, so
`MissingField` appears instead); a non-canonical encoder (AC-14 literal bytes).

Rust idioms are otherwise clean: typed `Serialize` records instead of `serde_json::Value`, an
exhaustive `match` with no catch-all in both `From` impls, no panic tokens, checked `try_from`, and
a `//!` header and `///` on every public item. The shared `render_witness` refactor of
`function.rs` is behavior-preserving: same format string, and bindings now pass as `(&str, i64)`.

## New findings (disposition pass 1)

Reviewed at 33d2f1205611b644ff2c41a5c1307bbcbe02159e (fix a86a610 plus a merge of `main` bcce798, #277).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | medium | `kani/generate/frame.rs` re-exports `oracle::scalar::{bound_members, literal}` under the aliases `graph_bindings` and `graph_literal`, so that `replay/state_clause.rs` can call them. AD-004's dependency direction gives `replay --> kani, core` and says "A directory imports only the directories to its right", so oracle is excluded. `routed` lists `oracle` explicitly, which shows the arrows are not meant to be transitive. The alias launders an oracle import through kani. `tests/it/layout.rs` cannot see it, because it reads only the `crate::kani::...` path. The fix is small: move `operation_declaration`'s FR-341 name and level decoding into `kani::generate::frame`, for example as a `ClauseShape` method that the generator owns, since it already decodes the clause's parameters, and drop the re-export | src/kani/generate/frame.rs:53-55, src/replay/state_clause.rs:43, src/replay/state_clause.rs:869-895 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | The behavior is unchanged: a post state outside the range still ends as `Refused(Admission(invalid-value))`, which reads as `Inconclusive(ReplayRefused)`, re-probed at 33d2f12. The decision belongs to the owner. The leader routed it to the planner and QSL, and FR-024 Current state now records it neutrally as an open known gap, with no code and no criterion approving it. Cite the routed ticket id in that bullet once it exists |
| FND-002 | fixed a86a610 | The AC-17 test puts the out-of-range value in the playback alone and in the post state alone, and requires the post-only case to build. The swapped-check mutant now fails it |
| FND-003 | fixed a86a610 | The AC-13 test adds a second call with a distinct population, key and type, and four distinct members on each label, all checked against the inputs. The hard-coded-key mutant now fails it |
| FND-004 | fixed 33d2f12 | The local reader is gone. The replay uses the generator's `Graph`, `ClauseShape::read` and #277's `field_range`, so there is one range reader. A member with no i64 range is carried unchecked with no `DeclaredDomain`, which matches `state_domains`, where the generator draws such a field without an assumption. QSL's `replay_state_clause` reads no `declared_domains`. It is stated in FR-024 Behavior and tested with `fixture_with_unbounded_balance`. The oracle helper re-export it introduced is new FND-009 |
| FND-005 | fixed a86a610 | `tc_035_the_envelope_declares_each_ranged_field_and_the_transcript_names_the_playback` pins the exact transcript, each domain's parameter, path and bound, and path [1] when `balance` is unranged |
| FND-006 | fixed a86a610 | `bind` refuses an undeclared name (`UndeclaredField`) and a repeated one (`DuplicateField`) on both sides, tested on both sides, and the error lists in FR-024, FR-029, interface-001, AD-002, TC-035 and TC-040 are updated |
| FND-007 | fixed a86a610 | `ModelError::{NotAnAddress, Unreadable, WrongDigestDomain, NoOwner, Ambiguous}` with a `sha256-jcs` domain check, each case covered by the `src` unit test `tc_035_the_model_header_is_the_one_owner_or_a_distinct_error` |
| FND-008 | accepted-no-change | Part (a) is fixed in a86a610: `digest_text` over `DigestRecord::{domain, hex}` replaces the local `hex()`. Part (b), each document encoded twice, is kept: three small documents per replay under `PREIMAGE_LIMITS`, the cost is negligible, and a fix would widen `core::canonical`'s API for no behavior |

Round 1 evidence, at 33d2f12 in my own worktree and target dir:

- `make ci` exits 0. lib: 167 passed. it: 360 passed, 24 ignored. doctest: 1, in both the msrv and
  the test passes. deny (advisories, bans, licenses, sources) ok. `make kani` was not run.
- Swapped-range mutant: `tc_035_a_value_outside_its_declared_range_is_out_of_domain` FAILS, as it
  should.
- Hard-coded-key mutant: `tc_035_the_documents_hold_the_playback_the_post_state_and_their_digests`
  FAILS, as it should.
- Out-of-range post probe: the result is still
  `Ok(Inconclusive(ReplayRefused(InvalidRuntimeInput)))`, so FND-001 is deferred, not fixed.
- Merge resolution: the PR's delta to `src/kani/generate/frame.rs` against `main` bcce798 is
  visibility only (`Graph`, `ClauseShape`, `field_range`), plus the FND-009 re-export and a doc
  line. `Shape` keeps both `result` and #277's `condition_field`/`member`. Fixture variants 30, 31
  and 32 are unique; the duplicate `variant: 1` already exists on `main`.
- Each mutant and probe was reverted, and `git status --porcelain` is clean.

## Dispositions (round 2)

Reviewed at 9b26d3a44f3b02f2fe18f6fde273bcb4a74fb08e. The diff from 33d2f12 touches only
`src/kani/generate/frame.rs`, `src/replay/state_clause.rs` and `tests/it/layout.rs`.

What I measured:

- The re-export is gone.
- `replay/state_clause.rs` imports only `crate::core`, `crate::kani` and `crate::replay`, and
  `grep crate::oracle src/replay/` is empty.
- The new `ClauseShape::declared_parameters` calls `oracle::scalar::{bound_members, literal}`
  inside `kani`. That is kani's own `kani --> oracle` edge, which AD-004 allows. It is not
  re-exported, and replay names no oracle item, so the rule that a directory imports only the
  directories to its right holds.
- `l_2_no_file_but_the_crate_root_re_exports_an_item` passes on the head, so it does not
  false-fail on existing code: no `pub use` or `pub(..) use` exists outside `lib.rs`. With the old
  re-export put back in my worktree it FAILS and names the line. Reverted; the tree is clean.
- `make ci` exits 0: lib 167, it 361 with 24 ignored, doctest 1, deny ok. `make kani` was not run.
- Strict coverage: 44 unbacked rows, the same set as round 1 (`main` has 66).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-009 | fixed 9b26d3a | The oracle re-export is removed. The FR-341 decode moved to `ClauseShape::declared_parameters` in `kani::generate::frame`, and a layout test now refuses any directory-level re-export |

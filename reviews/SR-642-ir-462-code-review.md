---
id: "SR-642"
title: "CG PR 207 code review: bounded-corpus case identity from the canonical request"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@68caecc4bcd01a0049802275f06f8d5613475461; src/bounded_kani_corpus.rs, schemas/kani-corpus-proof-graph-v1.schema.json, tests/it/bounded_kani_corpus.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-023
    type: references
---

# SR-642: CG PR 207 code review

## Summary

Ticket: IR-462. PR: agent-ix/quire-contract-codegen#207, head 68caecc, base main 8fcb51f. This is
code-review with the rust-review lane folded in, scoped to `git diff origin/main...HEAD`.

## Method

I read the whole diff and all of `src/bounded_kani_corpus.rs` at the head. I read the Contract IR
kani types at the locked IR commit 54f9a48: `FiniteInput`, `ProfileSelection`,
`CheckedArithmeticRequest`, `GraphRequest`, `CollectionQuery` and `QueryKind`. I read
`qsl_foundation::digest::ByteDigest` at the locked QSL commit 628d378, and `deterministic_json`
and `normalize_dependencies` in `src/kani.rs`. I grepped the repo for old `{family}_{n}` names and
for other digest uses. I ran `make ci` at the head and 14 mutation probes on the identity. The
probes are listed in SR-643.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `RequestIdentity::from` copies the IR request fields one by one through field access, so it does not destructure them exhaustively. The IR request structs have public fields, are not `#[non_exhaustive]`, and come from IR `main`. If a field is added upstream, this code still compiles and leaves the field out of the identity. Two distinct cases would then share a name: inside one registry the second is refused, and across runs with fresh registries it overwrites the first, which is the bug IR-462 fixes. No test catches the omission (SR-643 FND-001). Writing `let CollectionQuery { source_id, values, max_items, kind } = request;` (and the same for the other two structs) turns a new field into a compile error. | src/bounded_kani_corpus.rs:208-237 |
| FND-002 | low | The identity is not canonical over the finite input. `CaseIdentity` serializes `FiniteInput` as given, including the order of `objects` and `references`. `render_graph_oracle` sorts the edges (line 449), so two inputs that differ only in reference order render the same oracle under two different names. The two copies are emitted side by side and the collision refusal does not fire. The doc comment (line 138) and TC-023 call this the "canonical request content". Either sort `objects`/`references` before hashing, or say the identity is over the input as supplied. | src/bounded_kani_corpus.rs:166-172, src/bounded_kani_corpus.rs:346-353 |

## Verdict

Mergeable once FND-001 is fixed. It is a one-screen change, and it should land with this PR
because the PR is what introduces the hand-written mirror. FND-002 is low.

What is right:
- `deterministic_json` is `serde_json::to_vec` plus a newline, not a canonical (JCS) encoder. Over
  these types it is still deterministic. Every hashed type is a struct, a `Vec`, a `&str`, an
  integer or a unit/newtype enum. There is no `HashMap`, no float and no map with non-string keys,
  and the field order is fixed by the struct declaration. `i128` serializes natively in
  serde_json 1.0.151. The census is normalized (sorted by `proof_id`, ids unique after
  validation). `original_path`/`replacement_path` are dropped by normalization, and the
  Required-only rule forces them to be absent anyway.
- At the locked IR the mirror is complete. Arithmetic has 6 of 6 fields, graph has 5 of 5,
  collection has 4 of 4, and the `QueryKind` and `NumericOperator` matches are exhaustive.
  Everything the emitted artifacts depend on (family, the lowered value, the oracle body built from
  the request and the input references, and the census) is a function of what is hashed.
  `DispatchIndex` and the capability matrix can only refuse a case, never change its artifacts.
- The claim comes after every fallible step: census validation, the identity computation and all
  three lowerings return before `emitted.claim`, and `render_artifacts` is infallible. A refused
  case claims nothing, and a duplicate is refused with `InvalidInput`
  `kani_corpus_identity_collision`, which matches interface-001's existing text.
- `ByteDigest::of` is plain SHA-256, and its `LowerHex` gives exactly 64 lowercase hex digits, which
  matches the schema. `qsl-replay` is already the crate's only QSL dependency, and
  `src/spine_replay.rs` already uses `ByteDigest`. No new crate or hash code is added, and `sha2`
  stays a dev-dependency. This is the repo's allowed kind of hash (a canonical identity digest that
  binds a proof to its content), not a pin or file record.
- The proof symbol is `corpus_case_{family}_{64 hex}`, at most 87 characters. It starts with a
  letter and is a valid Rust identifier. The longest artifact file name is about 99 bytes. The
  three real-Kani corpus tests pass with these names.
- There is no `unwrap`, panic or cast beyond the existing `expect` on infallible serialization,
  which the module already uses for the proof graph. No compatibility shim is added for the old
  counter names, and none are left in the repo. The corpus replay retired in #205 is not re-added.

Conflict note: IR-464 will add `kani::cover` to the corpus harnesses in `render_artifacts`. This PR
does not touch `render_artifacts`, so the conflict is limited to the test module, if IR-464 adds
tests at the end of it.

## Gate

`make ci` at 68caecc, with a private scratchpad TRUSTED_HOME and
`CARGO_TARGET_DIR=<worktree>/target`, exited 0. fmt-check, spec, clippy `-D warnings`, the MSRV
test, deny, audit-unsafe, rustdoc and test all passed. Each test run gave 96 unit and 230
integration tests passed, with 8 ignored. Those runs include the three real-Kani corpus tests
(`tc_023_kani_executes_the_generated_arithmetic_harness`, `..._graph_harness`,
`tc_023_kani_falsifies_the_generated_false_collection_harness`) and the three new unit tests.

## Dispositions

Round 1 was reviewed at 3c006b81292fb6e882649ba4d12e0bc9d78d54f5, rebased on main 5a924e1.
`make ci` exited 0 there, with a private TRUSTED_HOME. Each test run gave 101 unit and 238
integration tests passed, with 8 ignored, including the three real-Kani `tc_023_kani_*` tests.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 545d1bc: `CheckedArithmeticRequest`, `GraphRequest` and `CollectionQuery`, and also `FiniteInput`, `FiniteObject` and `FiniteReference`, are destructured without `..`. A new field upstream is now a compile error. |
| FND-002 | fixed | 545d1bc: `InputIdentity` sorts the object and reference tuples before hashing. Removing either sort makes `tc_023_identity_is_canonical_over_the_input_population_order` fail. |

Round-1 mutation probes were scratch edits, reverted afterwards. Of 20 probes, 18 were caught.
Two survived, and both are equivalent mutants:
- Removing the top-level `profile` survives. `input.profile` is hashed inside `InputIdentity`, and
  the guard at line 390 refuses `kani_profile_input_mismatch` before the identity is computed, so
  the two values cannot differ on any case that reaches the hash. The field is redundant but
  harmless.
- Adding `dedup()` after both sorts survives. IR `FiniteInput::validate` (IR 54f9a48, `abi.rs`)
  already refuses a duplicate object identity (`kani_population_invalid`) and a duplicate
  `(source, field, target)` reference (`kani_reference_invalid`), so no validated input has a
  duplicate. Not deduplicating is correct: the sort is enough, and nothing distinct is merged.

---
id: "SR-750"
title: "CG PR 223 code review (Rust lane): AD-004 step 2d-0, core/naming.rs"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@c6e5186329824f3ef725be50ef1cb85688941842; src/core/naming.rs (new), src/core/artifact.rs, src/core/mod.rs, src/oracle.rs, src/lib.rs, and the import-only edits in bound, bound_strategy/{census,generation,population}, composite_equality, exact_function, exact_scalar, harness, kani, kani_obligations, state_frame, strategy"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: references
---

# SR-750: CG PR 223 code review (Rust lane)

## Summary

Ticket: IR-348 (the commit message says IR-344). PR: agent-ix/quire-contract-codegen#223 at
c6e5186, one commit on main 224ca6e. Methods: code-review with the rust-review lane folded in,
a `git diff -M --color-moved` motion check, a text diff of the removed `oracle.rs` lines against
the new `core/naming.rs`, a public-API comparison (crate-root `pub use` list and the rustdoc
`all.html` item set, base against head), a private-items rustdoc pass, an independent
byte-identity dump in two uncommitted scratch worktrees outside the repo (deleted afterwards),
and both gates run by this reviewer.

Checked and found as claimed:

- Pure motion. The 192 lines removed from `oracle.rs` and the body of `core/naming.rs` differ in
  exactly one line, `fn observation_name` to `pub(crate) fn observation_name`. The removed block
  is the nine named helpers plus the whole `#[cfg(test)] mod tests` (three tests, trace tags
  `FR-022-AC-9, TC-033` unchanged). `MAX_GENERATED_SOURCE_BYTES` and its doc line moved verbatim
  to `core/artifact.rs`, still `pub`. `dependency_parameters`, `typed_dependency_parameters`,
  `reference_key` and `dependency_key` stay in `oracle`. Every other hunk is a `use` path edit
  (plus the `kani.rs` inline path `crate::oracle::readable_name_component`) and one module-doc
  line in `core/mod.rs`. No shim or alias at the old paths; no reference to an old path remains
  in `src`, `tests` or `spec`.
- Public API. The crate-root `pub use` name set is unchanged; only `MAX_GENERATED_SOURCE_BYTES`
  is now re-exported from `crate::core::artifact`. The rustdoc `all.html` item list is
  identical base and head; the only file-set difference is rustdoc's redirect stub for the
  re-exported constant (`oracle/constant...` to `core/artifact/constant...`), and the root
  constant page text is identical.
- Direction. `core/naming.rs` imports only `std` and `quire_contract_model::StateObservation`,
  nothing in this crate, matching `core/mod.rs` ("`core` imports nothing else in this crate").
  Every user of a moved item (oracle, bound, strategy, harness, kani, kani_obligations,
  state_frame, exact_scalar, exact_function, composite_equality, bound_strategy) sits above
  `core`. No new edge into a non-core module.
- Byte identity, independent of the coder's Debug dump. Same throwaway instrumentation applied
  to base 224ca6e and head c6e5186: a hook in `Artifact::new` (path and contents of every
  generated file), in `ArtifactBundle::new` (the bundle's `serde_json` text, the path the coder
  did not dump), and at the return of `unique_names`, `oracle_symbol`, `reference_identifier`
  and `readable_name_component` (inputs and outputs). `cargo test --locked` on each (115 unit,
  246 integration, 1 doc test, all pass). Sorted dumps: 19,746 records (7,095 artifacts, 79
  bundle JSON texts, 877 oracle symbols, 8,952 readable components, 2,280 reference
  identifiers, 463 unique-name sets), 169,689,492 bytes each side, sha256
  `ab548c5cfb92a3c1813eb88e62120b02c7b4ab90df283d61d29b52f649a16a23` both, `cmp` identical.
- Rust idioms. `observation_name` at `pub(crate)` is the minimum visibility for `reference_key`
  in a sibling module. `rust_component` stays private. No new `unwrap`, cast, `unsafe` or
  allocation; the moved bodies are unchanged.
- Gates, run by this reviewer at c6e5186: `make ci` exit 0 (fmt-check, spec, clippy
  `-D warnings`, msrv, deny, audit-unsafe, rustdoc, test: 115 + 246 passed, 9 ignored, 1 doc).
  `make kani`: exit 0, 9 passed (see Verdict).

## Verdict

PASS with two low findings, neither blocking. The change is a pure definition move with one
stated visibility edit, the public API is unchanged and generated output, including the bundle
JSON, is byte-identical. FND-001 is a doc link the move broke in a private item, which
`make rustdoc` (public items only) does not check. `make kani` at c6e5186, run by this reviewer:
exit 0, 9 passed, 0 failed (1222.5 s). A first attempt was terminated by an outside signal while
it was still queued on the host-wide kani lock, before any test ran, and was rerun to completion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Broken intra-doc link from the move: the doc of `generate_named_boolean_oracle` says "several oracles together through [`unique_names`]", and `unique_names` is no longer in scope in `oracle.rs` (it imports only `observation_name`, `oracle_symbol` and `reference_identifier` from `core::naming`). `cargo doc --document-private-items` reports "unresolved link to `unique_names`" at head and not at base; `make rustdoc` misses it because the item is `pub(crate)`. Fix as a path fix: `[`unique_names`](crate::core::naming::unique_names)` | src/oracle.rs:147-148 |
| FND-002 | low | Split imports of one module. In three files the PR adds `use crate::core::{artifact::MAX_GENERATED_SOURCE_BYTES, naming::...};` beside the existing `use crate::core::artifact::Artifact;`, so `crate::core::artifact` is imported in two statements, while the other files of the same PR merge it as `core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES}`. Merge into `use crate::core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES};` and `use crate::core::naming::{...};` | src/composite_equality.rs:74-76, src/exact_function.rs:127-132, src/exact_scalar.rs:48-53 |

## Dispositions

Round 1, reviewed at 71f7abea2dbea194cddb0af68f7e061d1441a5fc (PR rebased onto main 7529a46; the
reviewed c6e5186 is now 45489be). `git diff 224ca6e c6e5186` and `git diff 7529a46 45489be` are
byte-identical patches, and main changed no file under `src/`, `tests/` or the Cargo manifests
between 224ca6e and 7529a46, so the byte-identity and `make kani` evidence above carries over.
Fix commit 71f7abe touches only `use` lines in three files and one doc comment; no code path
changed. `cargo fmt --check` and `make lint` pass at 71f7abe. `cargo doc --document-private-items`
reports one unresolved link, `DeclarationRefusal` at src/composite_equality.rs:18, identically at
base 7529a46: pre-existing and outside the PR diff.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 71f7abe |
| FND-002 | fixed | 71f7abe |

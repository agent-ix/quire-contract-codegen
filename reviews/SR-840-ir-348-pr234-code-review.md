---
id: "SR-840"
title: "CG PR 234 code review: AD-004 step 2f, kani/ module move"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@5603204a15da7bfda7734dde63b8e92a7d7d3b4b; src/kani/** (33 files), src/kani_witness_join.rs, src/lib.rs, src/oracle/scalar/mod.rs, src/routed_generation.rs, src/spine_replay.rs, tests/{common/withdraw_fixture,exact_scalar_support/agreement_cases,it/bounded_kani_corpus,it/exact_scalar_generation,it/kani_argument_order,it/kani_obligations}.rs; base origin/main 1629715"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-840: CG PR 234 code review (code-review with the rust-review lane)

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#234 at 5603204, six commits on base
origin/main 1629715 (the merge base). A pure-motion PR. The review checks that only paths,
`use` lines, the visibilities the step 2f item map names and intra-doc links changed; that
generated output cannot have changed; that the public API is unchanged; and that the new module
layout is idiomatic. The Rust lane is folded in, as `rust-review` says. The PR body was treated
as a claim and re-measured.

Measured by the reviewer:

- Pure motion, line by line. A script collected every non-blank, non-`use` line of the 13 base
  files (`kani`, `kani_obligations`, `kani_execution`, `kani_transcript`, `kani_witness_join`,
  `state_frame`, `bounded_kani_corpus`, `bounded_kani_profile`, `bounded_collections`,
  `definedness_arithmetic`, `finite_reference_graphs`, `kani_census`, `kani_identity`) and of
  every file under `src/kani/` plus the flat `kani_witness_join.rs` at head, with `pub(super)`
  and `pub(crate)` prefixes stripped, and compared the two multisets. Code lines present on only
  one side are exactly: four signatures reflowed by rustfmt because their visibility widened
  (`harness_path`, `contract_contexts`, `render_contract`, `single_diagnostic`); two
  `crate::kani_identity::StateFrameProperty` and one `crate::kani::validate_dependencies` path
  shortened behind a `use`; the `include_str!` fixture prefix `../` to `../../` (reflowed);
  and the `#[cfg(test)] mod tests` / `mod real_capture` / `mod synthetic` wrappers of the new
  test modules. Comment lines on only one side are the divided `//!` headers, the six `mod.rs`
  comments, intra-doc link targets, old-path mentions, and one comment
  ("see kani_transcript" to "see `output::report`"). No logic, name, string literal or attribute
  changed. `"kani_execution::classify_kani_run"` is unchanged (AD-004 step 2f bullet 1).
- Item set. 490 item declarations (any depth up to 8 spaces) on each side. Every
  visibility change is one the item map names: `obligation_kind`, `lower_clause`, `clause_stem`,
  `symbols`, `contract_contexts`, `abi`, `call`, `symbolic_arguments`, `LoweredClause`,
  `ClauseOracle`, `Symbols`, `SlotContext`, `Abi` and `Abi::access` to `pub(super)` in
  `clause.rs`; `harness_path`, `record`, `artifact` in `record.rs`; `LoweredScalarClaim`,
  `scalar_stem`, `ScalarLoweringRefusal`, `lower_scalar_claim`, `derive_domain`,
  `unsatisfiable`, `render_scalar` in `scalar.rs`; `render_precondition`, `render_contract`;
  `validate_plain_identity`, `validate_path`, `single_diagnostic`; `HarnessView` and `view`;
  `fresh_report_path`, `remove_stale_report`, `read_report`; `Playback`, `select_assertion_block`,
  `read_block` and `DecodeFailure::new` to `pub(crate)`; the six D-7 helpers to `pub(crate)` in
  `test_support.rs`. Field widening matches the map too: `ClauseOracle.parameters`,
  `LoweredScalarClaim.operation` and the other unnamed `LoweredScalarClaim` fields stay private.
  `Parameter`, `oracle_function_symbol`, `slot` and `ScalarOperation` are unchanged.
- Generated output. No moved file uses `file!`, `line!`, `column!`, `module_path!`,
  `type_name` or `CARGO_MANIFEST_DIR`, so the module path of an item cannot reach generated
  text. With the item text verbatim, output bytes cannot change. The PR's hooked-constructor
  dump (7204 records, 166,593,498 bytes, sha256 `dcc5726e...f067`, base and `6c727d3` equal) is
  consistent with that and was not re-run; `5603204` changes only three doc-link paths over
  `6c727d3`, which was confirmed with `git show`.
- Public API. Root `pub use` name set: 272 names at base and at head, no name added or removed.
  No `pub mod` at the root on either side, and every new module is `pub(crate)`, so the root
  re-exports are the whole public surface. The rustdoc `all.html` comparison in the PR body was
  not rebuilt; the name-set equality above makes it redundant.
- `git diff -M`: the four `lower/` files 100%, `frame.rs` 99%, `bounded_kani_corpus.rs` 98%,
  `identity.rs` 96%, `census.rs` 94%, `kani.rs` to `v1_bundle.rs` 69% (a split). The other new
  files are splits, covered by the line comparison.
- `mod.rs` files. All six hold only `pub(crate) mod` lines, their comments and a `//!` header;
  `test_support` is `#[cfg(test)]`. No `pub use`, no glob, no shim at an old path, and no
  `spec.rs`, `render.rs` or `terminal.rs`.
- Direction. `launch.rs` imports nothing of the crate outside its tests. `classify.rs` imports
  `identity` and `output` only (the doc links to `run::execute` are links, not imports, and were
  links at base). `test_support.rs` imports `core`, `abi`, `identity`. No family file
  (`scalar`, `clause`, `precondition`, `contract`, `record`, `outcome`) imports `negotiate`.
- History comments. The added lines carry no "moved from", "formerly" or step-number clause;
  `lib.rs` drops the older "(AD-004 step 2b/2c)" clauses. The INTERIM headers of `v1_bundle.rs`,
  `census_validation.rs` and `output/playback.rs` name the step that removes them.
- `Implements:` tags. The tag at `negotiate_kani_obligations` moved with it. The `lib.rs`
  module tags moved to `kani/mod.rs` and `generate/mod.rs`: `identity` FR-015, `generate` FR-015
  (was `kani_obligations`), `frame` FR-015 with its IR-412 note, `classify` and `run` FR-017
  (the two halves of `kani_execution`). `output` gains an FR-017 tag that `kani_transcript` did
  not carry; it is accurate (its tests already trace FR-017-AC-5, -AC-12, -AC-20), so it is noted
  here, not filed.
- Gates. `cg-2f-ci.log` ends `head=5603204... exit=0` (quire validate, fmt, clippy, 115 unit +
  252 integration (9 ignored) + 1 doc test, deny, unsafe audit, rustdoc `-Dwarnings`).
  `cg-2f-kani.log` ends with 9 Kani tests passed and `head=5603204... exit=0`. The gate worktree
  is at 5603204 with a clean status. Neither gate was re-run.

## Verdict

PASS, no findings. The move is verbatim at the line level, visibilities and fields are exactly
the item map's, the public API name set is unchanged, generated output cannot change, the
module layout is idiomatic and the gates pass on the final head. The comment-only edits under
`tests/` are a gap against AD-004's text and are filed in the gap analysis (SR-841), not here.
New doc-comment lines from the intra-doc link rewrites run past 100 columns (for example
`census_validation.rs` "Shared by [`generate_kani_bundle`](super::v1_bundle::...)"); rustfmt does
not wrap comments in this repo and the gate passes, so this is not filed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

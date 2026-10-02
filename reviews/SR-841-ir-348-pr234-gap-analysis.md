---
id: "SR-841"
title: "CG PR 234 gap analysis: AD-004 step 2f item map and decisions D-1 to D-8 against the code"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@5603204a15da7bfda7734dde63b8e92a7d7d3b4b; spec/assurance/AD-004-cg-crate-layout.md (Step 2f item map: counting rule, What the move may and may not change, D-1 to D-8, Interim exceptions, Item tables, What a reviewer checks for 2f) checked against src/kani/**, src/kani_witness_join.rs, src/lib.rs, src/spine_replay.rs, src/routed_generation.rs, src/oracle/scalar/mod.rs and tests/ at the reviewed sha, base origin/main 1629715"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-841: CG PR 234 gap analysis

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#234 at 5603204. Planless run. The
acceptance criteria are AD-004's step 2f item map: its rules on what the move may change, the
decisions D-1 to D-8, the item tables and the closing "What a reviewer checks for 2f" list. Each
was checked against the code at the reviewed sha, not against the PR body.

Examined, with the result:

- Item tables. A script read every table row of the step 2f item map and checked that each named
  item is defined (`fn`, `struct`, `enum`, `const`, `static`, `type`, `trait`, `impl` or
  `macro_rules!`) in the row's destination file. Every row resolves (the one miss was the
  parser reading "the `macro_rules!`" as a name; `macro_rules! capture` is at
  `src/kani/classify.rs:424`). Item declarations total 490 on each side.
- Test placement. `#[test]` counts per file: `clause` 1, `scalar` 4, `negotiate` 3 (8, as
  `kani_obligations` had); `classify` 17 (13 `real_capture` + 4 `synthetic`), `output/report` 2,
  `output/playback` 1, `run/execute` 5, `run/launch` 10, `run/harness` 1, `run/tool` 1,
  `run/report_file` 1 (40, as `kani_execution` + `kani_transcript` had once the two `#[test]`
  strings inside the playback constants are discounted); `kani_witness_join.rs` keeps 15.
- D-1. `HARNESS_MARKER`, `CHECK_MARKER`, `CheckKind`, `check_clause`, `select_assertion_block`,
  `read_block`, `unescaped_quote`, `bracketed`, `concrete_entries`, `Playback`, `DecodeFailure` and
  its `new` are in `kani/output/playback.rs`; the decode half and all 15 tests stay in the flat
  `kani_witness_join.rs`, which imports the scan half from `kani::output::playback`.
- D-2. `kani/abi.rs` holds the binding roles, primitive types, bounds, `KaniSolver`,
  `i64_literal`, `adapter_options`, `readable_component`. `KaniSubjectBinding` and the bundle are
  in `v1_bundle.rs`; `validate_dependencies`, `KaniDiagnostic`, `KaniErrorCode`, the three helpers
  and `deterministic_json` are in `census_validation.rs`. Both carry INTERIM headers naming the
  step that deletes them (4f; 4g or the V2 census input, and 1a).
- D-3. `outcome.rs` holds the 13 vocabulary items and imports no family; `negotiate.rs` holds the
  entry, the classifiers, the passes and `render`; `record.rs`, `clause.rs`, `scalar.rs`,
  `precondition.rs`, `contract.rs` hold their rows. No family file imports `negotiate`.
  `generate/mod.rs` holds declarations only.
- D-4. `classify.rs` holds the seven classification items and imports only `identity` and
  `output`. `run/` has `tool`, `harness`, `launch`, `report_file`, `execute`; `launch.rs` imports
  nothing of the crate outside its tests; `read_file` is in `execute.rs`.
- D-5. `output/report.rs` holds the report parse; `counterexample_playback` and the four
  `PLAYBACK_*` constants are in `output/playback.rs`.
- D-6. `real_capture` in `classify.rs` holds 13 tests and the 5 helpers; the fixture path is
  `../../tests/fixtures/kani-report/`, and only `classify.rs` includes fixtures. The three
  `mutated` parse tests are in `real_capture`.
- D-7. `kani/test_support.rs` is `#[cfg(test)]` in `kani/mod.rs`, holds `state_frame_harness`,
  `report`, `PASSED`, `COVER_OK`, `COVER_NO`, `discover_scratch` at `pub(crate)`, and imports
  `core`, `abi`, `identity` and no generator. `classify.rs` has `synthetic` and `real_capture`;
  the precondition no-checks test is in `run/execute.rs`.
- D-8. `state_frame.rs` is `generate/frame.rs` (99%), the corpus is
  `generate/corpus/bounded_kani_corpus.rs` (98%), the four thin files are under
  `generate/lower/` (100%); `impl StateComparison` stays in `frame.rs`.
- What may change. Only `use` lines, module paths, the map's visibilities and fields, divided
  `//!` headers, INTERIM sentences and intra-doc links changed in the moved items (line-level
  multiset comparison; see SR-840). `"kani_execution::classify_kani_run"` is unchanged.
- Files outside the move. `spine_replay.rs`, `routed_generation.rs` (including the test module's
  `kani::abi::KaniSolver`), the corpus's `validate_dependencies` calls, the flat
  `kani_witness_join.rs` and the `oracle/scalar/mod.rs` doc links are rewritten as listed. The
  `lib.rs` `pub use` line for `decode_falsification` and `DecodeFailure` is split in two. The root
  re-export name set is unchanged (272 names).
- Reviewer checklist. The six `mod.rs` files hold declarations only; `spec.rs`, `render.rs` and
  `terminal.rs` do not exist; each interim file names its deleting step; `make test` passes on the
  head (`cg-2f-ci.log`); generated output cannot change (no location-sensitive macro in any moved
  file, item text verbatim). The one item that does not hold is "no file under `tests/` edited":
  FND-001.

Test-oracle strength: no test was added, removed or changed in code, and every test kept its
body and its fixtures, so oracle strength is unchanged by construction. The relocated
`include_str!` fixtures resolve to the same files, and the 115 + 252 tests pass on the head.

## Verdict

PASS with one low finding. Every decision D-1 to D-8 and every item-table row holds in the
code, the move is verbatim, and the public API is unchanged. FND-001 is a conflict between the
AD's text and six comment-only edits under `tests/`. The edits replace old-path mentions of
deleted files and are an improvement; the fix is in the AD's wording, not the PR, so the PR is
mergeable as is.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AD-004 step 2f says "The set of re-exported names does not change, so no file under `tests/` changes" and its reviewer checklist requires "`make test` passes with no file under `tests/` edited". The PR edits six files under `tests/` (the PR body says seven): comment-only rewrites of old-path mentions such as `src/kani_obligations.rs:1450` to the new files. No test code changed. The AD's rule reads as code-only but does not say so, so the step as merged contradicts its own spec text. | spec/assurance/AD-004-cg-crate-layout.md:895, spec/assurance/AD-004-cg-crate-layout.md:1213, tests/it/kani_argument_order.rs:2, tests/it/kani_obligations.rs:1833 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | dd224b3 (round 1, reviewed at dd224b3: `git diff 5603204..dd224b3` touches only `spec/assurance/AD-004-cg-crate-layout.md` (two lines) and adds the two SR files; the code tree outside `spec/` and `reviews/` is byte-identical to 5603204. AD-004:895 now reads "no test code under `tests/` changes (comment lines that cite a deleted source path may be updated)" and AD-004:1213 "with no test code under `tests/` edited, comment-only path updates aside". The PR's six `tests/` edits are all comment lines, so the step now agrees with its spec; the PR body now says six files.) |

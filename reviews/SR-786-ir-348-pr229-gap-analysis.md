---
id: "SR-786"
title: "CG PR 229 gap analysis: AD-004 step 2f item map completeness and executability"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@8280a5667498f2edae563a98f127eaec01ed98d3; spec/assurance/AD-004-cg-crate-layout.md (Step 2f item map, dependency direction), checked item by item against src/{kani,kani_obligations,kani_execution,kani_transcript,kani_witness_join,state_frame,bounded_kani_corpus,bounded_kani_profile,bounded_collections,definedness_arithmetic,finite_reference_graphs,kani_census,kani_identity,lib,spine_replay,routed_generation}.rs and tests/ at origin/main 2130cd9"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-786: CG PR 229 gap analysis

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#229 at 8280a56. This is a planless run. It
asks two questions: does the step 2f item map cover every item and every dependency, and can a
coder carry it out as a verbatim move with nothing left to decide? The map was checked against
the code at origin/main 2130cd9, not against the AD's own tables.

- Completeness. The items of every file were listed with a script that applies the AD's
  counting rule: column-0 items, plus 4-space items inside `mod tests`, with `impl` blocks,
  consts, statics, `type` aliases and `macro_rules!` included and `mod {..}` template strings
  excluded. Every count matches the AD:

  | File | Items (production + test) |
  | --- | --- |
  | `kani` | 38 |
  | `kani_obligations` | 70 + 12 |
  | `kani_execution` | 46 + 37 |
  | `kani_transcript` | 31 + 21, counting `macro_rules! capture` |
  | `kani_witness_join` | 19 + 15 |
  | `state_frame` | 36 + 2 |
  | `bounded_kani_corpus` | 22 + 31 |
  | `bounded_kani_profile` | 3 + 5 |
  | `bounded_collections` | 1 + 2 |
  | `definedness_arithmetic` | 1 + 2 |
  | `finite_reference_graphs` | 1 |
  | `kani_census` | 8 |
  | `kani_identity` | 16 |

  Every item name in the tables exists, and each is assigned exactly once. No item is missing
  and none is invented.
- Production dependencies. Every cross-file call in the split files was walked. The planned
  order holds for all production code:
  - `abi`, `census` and `identity`, then `generate` and `output`, then `classify` and `run`.
  - Inside `generate`: `outcome` is a leaf, since no `UnsupportedObligation` field names a
    family type. `record` imports only `outcome` and `core`. `scalar` imports `outcome`,
    `record` and `abi`. `clause` imports `outcome` and `abi`. `precondition` and `contract`
    import `clause`. `negotiate` imports all of these.
  - `run/harness.rs` reads `identity` only. `execute.rs` imports `launch`, `report_file`,
    `harness`, `tool` and `classify`. `classify` imports `output` and `identity`. The scan half
    of `playback.rs` imports nothing from `replay`.

  No item calls an item placed after it, with one exception in the order text (SR-785 FND-002).
  Each `pub(super)` and `pub(crate)` the tables state is needed by a real call site. Two field
  widenings go wider than needed (FND-002 and FND-003).
- Tests. Every test is placed by what it calls, with two exceptions (FND-001):
  - `test_support` helpers are used by more than one new file.
  - The `synthetic` tests call `classify_kani_run` only.
  - `a_precondition_harness_with_no_checks_...` calls `launch_evidence`, so it goes to `execute.rs`.
  - Tests that use `view` or `discover_from` follow those functions.
  - The 15 `kani_witness_join` tests use only `decode_falsification`, `argument_types`,
    `WitnessSchemaError` and `read_block`, which the D-1 `pub(crate)` covers.
  - The fixture path `../../tests/fixtures/kani-report/` is right from `src/kani/classify.rs`.
- `tests/` needs no edit. Nothing under `tests/` names a module path of a moved file. Every
  import is `quire_contract_codegen::<Name>`, and `lib.rs` keeps the names. The only mentions
  of `src/kani_obligations.rs` and similar files are comments, and the AD's "no file under
  `tests/` changes" leaves them as they are.
- Files outside the move. `lib.rs`, `spine_replay.rs` and `routed_generation.rs` are listed,
  but one test import in `routed_generation.rs` is missing (FND-004). The 2e convention of
  `pub(crate) mod` in `mod.rs` files is what makes `lib.rs`'s new `pub use kani::generate::...`
  paths resolve. The AD inherits it through "as the 2e `mod.rs` files do".

## Verdict

Not mergeable until FND-001 is fixed: the test table contradicts the AD's own D-6 rule, and a
coder who follows the table gets a compile error. FND-002 also makes `make lint` fail as written.
Both fixes are one cell each. After them, the map can be carried out as a verbatim move.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Two tests the table sends to `output/report.rs` as "report types only, no fixture" call `mutated`. `mutated` reads `capture!("verified-satisfied-cover")`, a fixture, and by D-6 and the real_capture row it lives in `classify.rs`'s `real_capture` module. The two tests are `tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused` (src/kani_transcript.rs:844, `bad`/`no_location`/`unknown` are `mutated(..)`) and `tc_027_an_unnamed_check_category_is_a_property` (:876). In `report.rs` they do not compile, and `report.rs` may not import `classify`. Move both to the `real_capture` row: `report.rs` then has 2 tests and `real_capture` 13, and the 21 total is unchanged. Fix the D-6 sentence that counts them. | spec/assurance/AD-004-cg-crate-layout.md:1107-1108, :975-981 |
| FND-002 | medium | The `LoweredScalarClaim` row widens "fields pub(super)", which covers all of them, including `operation: ScalarOperation`. The next row keeps `ScalarOperation` private. A field reachable in `generate` with a type usable only in `generate::scalar` triggers `private_interfaces`, and `make lint` (`clippy -- -D warnings`) fails on it. Reproduced with rustc (`-D warnings`): "type `ScalarOperation` is more private than the item `Lowered::operation`". `negotiate` reads only `node_id`, `operation_identity`, `module_symbol` and `harness_symbol` (`Outcome::disposition_without_harness`, `assign_names`), and those four are the narrowest set the AD promises. | spec/assurance/AD-004-cg-crate-layout.md:1057, :1059 |
| FND-003 | low | The `LoweredClause`/`ClauseOracle`/`Parameter`/`Symbols` row widens every field and `Parameter` itself. The code outside `clause.rs` never reads `ClauseOracle.parameters` or any `Parameter` field: only `abi` and `call`, both in `clause.rs`, use them. If `parameters` stays private, `Parameter` can stay private too. The over-wide form compiles, so "the coder confirms by compiling" will not catch it, and it ships wider than the narrowest the AD claims. | spec/assurance/AD-004-cg-crate-layout.md:1053 |
| FND-004 | low | The list of outside files whose imports are rewritten gives `routed_generation.rs` as "(`kani_identity`)". Its test module also imports `kani::KaniSolver` (src/routed_generation.rs:400), which becomes `kani::abi::KaniSolver`. The corpus's inline call `crate::kani::validate_dependencies` (src/bounded_kani_corpus.rs:430) is also an item-body path that has to change, and D-8 implies it without naming it. The compiler finds both, but the map says the PR "has nothing left to decide". | spec/assurance/AD-004-cg-crate-layout.md:888-892, :998-999 |
| FND-005 | low | D-2 and the reviewer checklist require each interim file (`v1_bundle.rs`, `census_validation.rs`) to carry a new INTERIM header naming the step that deletes it. "What may change" allows only the `//!` header of a split file, "divided between its successors". A new header sentence is not in that list. Add it so that the verbatim-move check does not flag the header. | spec/assurance/AD-004-cg-crate-layout.md:874-878, :920, :1198 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | The fix split the `LoweredClause`/`ClauseOracle`/`Parameter`/`Symbols` row into a 3-item row and a `Parameter` row, but the sum line still reads "17 (`clause.rs`: 4 + 8 + 3 + 2)". The rows are now 3 + 1 + 8 + 3 + 2. The total of 17 is still right, but the addends name a row that no longer exists. | spec/assurance/AD-004-cg-crate-layout.md:1085 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a91ed1b (round 1, reviewed at a91ed1b: both mutated-using tests are in the real_capture row, which now has 13 tests; report.rs has 2 and playback.rs 1, total 21; D-6 names all three parse tests that call mutated; this matches the code at 2130cd9) |
| FND-002 | fixed | a91ed1b (round 1: LoweredScalarClaim widens only node_id, operation_identity, module_symbol and harness_symbol, so the private ScalarOperation field stays private and private_interfaces no longer applies) |
| FND-003 | fixed | a91ed1b (round 1: ClauseOracle.parameters stays private and Parameter gets its own row, private; only abi and call in clause.rs read it) |
| FND-004 | fixed | a91ed1b (round 1: the outside-files list adds routed_generation's test import kani::KaniSolver and the corpus's inline crate::kani::validate_dependencies calls) |
| FND-005 | fixed | a91ed1b (round 1: "What may change" allows the INTERIM header sentence in v1_bundle.rs and census_validation.rs, and widens only the fields a row names) |
| FND-006 | fixed | 81fcea6 (round 2, reviewed at 81fcea6: the sum line reads "17 (`clause.rs`: 3 + 1 + 8 + 3 + 2)", which matches the row split; the diff a91ed1b..81fcea6 is that one line only; every other table sum re-checked: kani.rs 38, obligations 70+12, execution 46+37, transcript 31+21, witness_join 19+15) |

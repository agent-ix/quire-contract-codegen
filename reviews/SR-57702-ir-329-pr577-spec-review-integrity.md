---
id: SR-57702
title: "IR-329 PR 577 Kani module ownership map integrity analysis"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@21f28b1bec5df6471dd6735214ce3145545d21a4; spec/spec.md (Kani module and public-surface ownership map, four added rows)"
---

## Summary

Ticket: IR-329. PR: quire-contract-codegen#577 at 21f28b1b. This integrity pass checks the
map's coverage, single ownership per concept, consistency with AD-004's layout, and the
precision of the retirement citations.

## Examined scope

- Completeness: examined. Every file under `src/kani/` on main now falls under some row.
- `kani/generate/lower/mod.rs`: examined. It holds the non-test function `admit_family` and
  tests traced to FR-015-AC-82, AC-86 and AC-88. Existing row 6 already owns it and retires
  it at AC-111.
- `kani/mod.rs`: examined. It holds module declarations and a `#[cfg(test)]` macro, with no
  `pub use` re-exports. The crate root's re-exports are in `lib.rs`, so the row's
  "module declarations" claim holds for `kani/mod.rs`.
- `generate/mod.rs`, `run/mod.rs` and `output/mod.rs`: examined. They hold module
  declarations and `#[cfg(test)]` helpers only.
- `kani/test_support.rs`: examined. Its users include `run/{execute,harness,launch,memory,namespace,report_file,scalar,tool}.rs`,
  `classify.rs`, `render.rs`, `scalar.rs`, `negotiate.rs` and the corpus.
- `role_parity_tests.rs`: examined. It is traced to FR-014-AC-38, FR-015-AC-60 and
  FR-015-AC-68, plus TC-024 and TC-025.
- AD-004 source layout and step 2f item map: context only. It lists `clause.rs` as "the V1
  clause lowering ... interim, reshaped by step 4c". It names neither `clause/v2.rs` nor
  `role_parity_tests.rs`.

## Verdict

CHANGES REQUESTED. The map is now complete, and no row of the base file was changed or
narrowed. Two new rows break single ownership or AD-004 consistency. Two citations are
imprecise.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The module-roots row claims the `lower` module root as "test-only support and module declarations" with no API. `lower/mod.rs` holds the non-test profile-admission function `admit_family`. Existing row 6 already owns `lower/mod.rs` (FR-015-AC-82 to AC-89, retiring at AC-111). That is double ownership with contradictory descriptions. Drop `lower` from the roots list. | spec/spec.md:140; spec/spec.md:135; src/kani/generate/lower/mod.rs:14 |
| FND-002 | medium | The map gives `kani/generate/clause/v2` its own V2 owner row, but AD-004's layout has no `clause/v2.rs` (nor `role_parity_tests.rs`). The module is declared inside `clause.rs`, which AD-004 and existing row 4 treat as V1-interim "pending AD-004 steps 4c to 4f". It is now unclear whether `clause/v2` retires or moves with its parent. Give it an AD-004 home, or note in the row that AD-004 does not yet place it. | spec/spec.md:132; spec/spec.md:133; src/kani/generate/clause.rs:4; spec/assurance/AD-004-cg-crate-layout.md:227 |
| FND-003 | low | The final row's owner cell is one compound phrase for three different things. Read literally, "the FR-014/FR-015 criteria traced by the tests" covers `kani/test_support`, which mostly serves FR-017 run, report and classify tests. Give each item its own owner: `role_parity_tests` → its traced FR-014/FR-015 criteria; `test_support` → the criteria of the tests that use it (FR-015, FR-017); module roots → each child module's owner. | spec/spec.md:140; src/kani/test_support.rs |
| FND-004 | low | The census row says `census_validation` and `corpus/mod.rs` retire under "FR-015-AC-110/111". AC-110, the collection slice, retires neither. `corpus/mod.rs` goes only with AC-111. AD-004 also lets the V2 census input replace `census_validation` instead. | spec/spec.md:134; spec/assurance/AD-004-cg-crate-layout.md:234 |

## Dispositions

Round 1, checked against fix-round head 59b9b4e031340a6642cdb160408c12f0f1b11d71.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The module-roots row now names only `kani/mod.rs` and `kani/{generate,run,output}/mod.rs`. It states that `lower/mod.rs` is owned by the finite-input auxiliary row, which keeps a single owner. |
| FND-002 | fixed | The `clause/v2` row states that AD-004 does not yet place this child of interim `clause.rs` and that its placement or retirement must be resolved with steps 4c to 4f. The `role_parity_tests` row states the same gap. |
| FND-003 | fixed | Each item now has its own row. `role_parity_tests` is owned by FR-014-AC-38 and FR-015-AC-60/68, which matches its trace tags. `test_support` is owned by the FR-015 and FR-017 criteria of its users; its users in run, output, classify and generate were verified. Module roots are owned by their children's owners. |
| FND-004 | fixed | The census row now ties `census_validation` to steps 4f/4g or replacement by the V2 census input. It ties `corpus/mod.rs` to FR-015-AC-111 only. |

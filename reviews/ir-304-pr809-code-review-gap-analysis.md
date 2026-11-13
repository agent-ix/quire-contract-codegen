---
id: SR-17940486709564847047752
title: "IR-304 PR #809 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #809 at 3c7845ca; IR-304 second PR; FR-022-AC-35, TC-033 step 28; spec/routed/matrix/tests.md, spec/routed/matrix/TC-033-routed-generation.md, tests/it/requirement_records.rs, tests/it/kani_obligations.rs, docs/qualification/ir304-kani-guard-native-harness.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304. Plan completion: not assessed. `quire matrix --scope .` at 3c7845ca binds
FR-022-AC-35 to one test, `tests/it/requirement_records.rs:1885`
`the_guarded_native_harness_is_thin_and_ends_with_its_cover` (status `tagged`). That test builds
both harnesses through `generate_routed_native` and asserts only their source text: native
representation, no `rt::Integer`, one guard assumption before the oracle call, and the
`cover_reached` / forget / cover tail. It does not run Kani. All new production code (two
`Statement::Forget` pushes in `render_scalar`) is owned by FR-022-AC-35 and FR-015-AC-7; no
unowned code, no stub, no tautology.

The verdict half of FR-022-AC-35 ("FR-017 execution under installed Kani reports
`CoverUnsatisfied`, not `Verified`" and "the same harness with the guard `x > y` reports
`Verified`") rests only on the qualification record. That record states the runs were raw
`cargo kani` on a generated file copied into a scratch crate by a throwaway, uncommitted test.
Its own "What this does not establish" section says: no `run_kani`-style launch, no FR-017-AC-63
two-phase run and no classification code ran; the FR-017 column was mapped by hand. TC-033's
procedure for AC-35 (unchanged by this PR) says "run both harnesses through FR-017 under
installed Kani and read the evidence outcome". That step was not run.

The CG runner path for exactly this shape already exists in the `make kani` lane:
`tests/it/kani_obligations.rs` `run_scalar_under_real_kani` (line 2452) runs a routed scalar
harness through `execute_kani_obligation`, and `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`
and `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` mutate the harness text and
assert the classified `KaniRunOutcome`. `tests/it/kani_render_private.rs` asserts
`KaniRunOutcome::CoverUnsatisfied { satisfied: 0, total: 1 }` the same way. Each AC-35 run took
8 to 9 s at 0.6 GB, so the lane cost is small.

Reviewer check of the record against the logs: the five scratch crates match the harness this
head generates except for three package-name comment lines, and the Kani logs show the stated
verdicts (see the code-review SR).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-022-AC-35 is marked `✅ Implemented` (routed `tests.md`), covered in FR-022 "Scoped evidence status" and TC-033 step 28, with method Test (TC-033). But no test in the repo runs either harness under Kani, and the recorded verdicts did not come from FR-017 execution: the record says CG's runner and classification never ran. The only tagged test checks harness text, which it could pass while the real verdict is `Verified` for the infeasible case. Fix in this PR by adding `#[ignore]` kani-lane tests, tagged FR-022-AC-35 / TC-033, that build both harnesses through `generate_routed_native` and run them through `run_scalar_under_real_kani` / `execute_kani_obligation`. They assert `CoverUnsatisfied { satisfied: 0, total: 1 }` for `x < y` and `Verified` for `x > y`, plus the three mutants as harness-text replacements (the existing `tc_025`/`tc_027` pattern), in a module inside `KANI_FILTERS`. Otherwise mark AC-35 Partial with the raw-run evidence, in the form of FR-015-AC-147/148 (spec-review SR FND-001) | spec/routed/matrix/tests.md:27, spec/routed/matrix/TC-033-routed-generation.md:120-121, spec/routed/matrix/TC-033-routed-generation.md:313-317, tests/it/requirement_records.rs:1883-1929 |
| FND-002 | low | The qualification record cannot be regenerated from the repo: the harness files were written by a throwaway test that was not committed, and the mutants are hand edits of scratch copies. A kani-lane test (FND-001) would make it reproducible. Until then the record should at least name the exact head the harness text was generated from | docs/qualification/ir304-kani-guard-native-harness.md:21-27 |

## Verdict

Not mergeable at 3c7845ca: FND-001 must be fixed in this PR, preferably with the runner test
because the helper and pattern already exist. Coverage of the text clause that AC-35 now carries
(native integers, forgets, cover last) is real and mutation-sensitive. The executed raw-Kani
results are genuine and match the generated text. They count as Demonstration evidence, not as
the FR-017 Test evidence the AC and TC-033 require.

## Dispositions

Round 1, reviewed head 1380afbe. `quire matrix` now binds FR-022-AC-35 to
`tests/it/kani_obligations.rs:2998` and `:3017` (ignored, kani lane) and to
`tests/it/requirement_records.rs:1885`. The reviewer ran both kani-lane tests through CG's
`execute_kani_obligation`: 2 passed, 75.14 s.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | `tc_033_native_guarded_harness_reports_cover_unsatisfied_and_verified` asserts `CoverUnsatisfied { satisfied: 0, total: 1 }` (`x < y`) and `Verified` (`x > y`) through CG's runner, and `tc_033_native_guarded_harness_mutants_are_detected` asserts mutants A and B `Falsified` and C `Verified`. Both are tagged FR-022-AC-35 / TC-033, inside the `kani_obligations` filter of `KANI_FILTERS`, and passed in the reviewer's run |
| FND-002 | fixed | The record now has a Reproduction section naming the two committed tests and the command, and states that the raw table is the harness text of head 3c7845ca. The throwaway-test sentence is gone |

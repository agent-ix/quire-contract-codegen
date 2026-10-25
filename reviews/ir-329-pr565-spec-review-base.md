---
id: SR-17011
title: "Base review of IR-329 AC-109 matrix status reconciliation (CG #565)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/matrix/tests.md (PR #565 diff at 5bbc92a0720172d6b706762991b13516605097ed against main 4eccd682545a240cf2289bf2040e820002bdf2e5)"
review_set: subset
date: 2026-10-25
---

## Summary

Ticket: IR-329. Base checklist review of the single-file diff in draft PR #565, which splits the
former combined FR-015-AC-82..95/109..111 rows of the Kani test matrix into an AC-82..94 row, an
AC-95 row, an AC-109 row and an AC-110/111 row (upper table), and splits the TC-023 trace row into
an AC-109 row and a remainder row (lower table). The AC-109 `✅` status is backed by the computed
matrix and by source inspection on CG main; no status is overstated, no criterion text changed,
and AC-95, AC-110 and AC-111 stay visibly `🚧 Planned` in both tables. One low finding: the
rewritten TC-023 remainder row's status prose omits FR-015-AC-23, which the row still lists.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | The rewritten TC-023 remainder row lists FR-015-AC-23 but its new status prose accounts for every other listed AC and says nothing of AC-23 (computed matrix: untagged; owned as planned by the AC-19..25 and TC-025 rows). The row is `🚧`, so nothing is overstated, but a reader cannot tell AC-23's state from this row. | spec/kani/matrix/tests.md:108 | correct-requirement-no-evidence |

## Evidence

- Merge base equals current `origin/main` (4eccd682), so the base tree is CG main.
- `quire matrix --scope . --format tsv` (quire 0.36.2, engine 0.50.2) at the PR head reports:
  FR-015-AC-109 `tagged` by `tests/it/kani_obligations.rs:2119` (`#[trace("FR-015-AC-109", "TC-023")]`,
  `admitted_integer_division_is_unrendered_and_emits_no_harness`, not ignored) and
  `tests/it/routed_generation.rs:2502` (`#[trace(..., "FR-015-AC-109", "TC-033")]`,
  `tc_033_an_underivable_claim_is_underived_and_carries_the_nodes_identity`, not ignored);
  AC-82, 83, 86, 88, 91, 94 `tagged`; AC-84, 85, 87, 89, 90, 92, 93, 95, 110, 111 `untagged`. Every
  tag claim in the new rows matches.
- The division test asserts `Unsupported(OperationNotRendered)` for `quire.op.integer.div` and no
  harness; the remainder test asserts `Underived`, a refused claim and no harness for
  `quire.op.integer.rem`. This matches AC-109's stated tests.
- Source search of `src/` and `tests/` finds none of `DefinednessArithmetic`,
  `prepare_checked_arithmetic`, `checked_method`, `render_arithmetic_`, `definedness_arithmetic`,
  `CheckedArithmeticRequest`; `src/kani/generate/lower/` holds only `bounded_collections.rs`,
  `bounded_kani_profile.rs`, `finite_reference_graphs.rs`, `mod.rs`; `BoundedCorpusRequest` has only
  `Graph` and `Collection` variants. The retained corpus replay (`replay_counterexample`,
  `compile_corpus_qsl_package`, `BoundedCorpusReplayError`) is present, as AC-109 requires.
- No-wrapper inspection: `CollectionQuery`, `GraphRequest` and `BoundedCorpusRequest` appear only in
  `src/lib.rs` re-exports and the corpus/lower modules; no `From` impl converts them.
- AC-95 row claim "generation still exports the bounded-corpus API" holds: `src/lib.rs:63-64`
  re-exports `generate_bounded_kani_corpus_case` and `BoundedCorpusRequest`.
- `make spec` exits 0; only pre-existing EARS warnings in untouched FR-017 and FR-014.

## Checklist and coverage

- No acceptance criterion text changed (diff touches only `tests.md`). AC-84/85 are described as
  "ended with arithmetic retirement (AC-109)", which is accurate: their own text is conditioned
  "Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic lowerer", and
  AC-109 records the coverage loss. They stay listed under a `🚧` row; nothing is narrowed or
  credited as covered.
- The AC-82..94 row moves from a stale "no criterion trace tags" to "Partial" with the computed tag
  list; it says "tags", not "covered", so tagged-but-unproven controls are not overstated.
- AC-95, AC-110, AC-111 each remain `🚧 Planned` with their gates (AD-004 step 4h, IR-805, graph
  replay) in the upper table, and stay listed in the `🚧` TC-023 row in the lower table. The new
  AC-109 lower row also states "AC-95 remains open". The matrix does not hide them.
- The zero-orphans end state of IR-329 is not claimed by this diff.
- No hash, digest, pin or tracking-record use is introduced. The PR body cites no SHA.
- IDs, TC ids and row structure are consistent with the surrounding table; FR-015's own AC-109 text
  already reads "Implemented", so the matrix now agrees with the requirement.

## Verdict

Clean apart from one low-severity wording gap (FND-001). The AC-109 status correction is
measured and correct, nothing is overstated or narrowed, and AC-95/110/111 remain visible as
planned. Observation, not a defect of this diff: one of the V2 serving criteria AC-109 cites
(FR-015-AC-117) is tagged only by an ignored test, and the TC-033 trace table in
`spec/routed/matrix/tests.md` does not list FR-015-AC-109 although the remainder test carries both
tags; both lie outside this PR's diff.

Tracker note for the dispatcher: Linear IR-329 was moved In Progress to Done at
2026-10-25T02:27:38Z (by the Peter Krenesky account), while the PR body states IR-329 remains In
Progress and its done condition (zero orphans both ways) is still open. This review did not change
the ticket state.
